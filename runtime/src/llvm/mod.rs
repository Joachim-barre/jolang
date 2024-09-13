use anyhow::{anyhow, Result};
use inkwell::{builder::Builder, context::Context, execution_engine::JitFunction, module::Module, types::{BasicMetadataTypeEnum, BasicType, IntType}, values::{BasicMetadataValueEnum, FunctionValue, PhiValue}, AddressSpace, OptimizationLevel, basic_block::BasicBlock};
use crate::Runtime;
use jolang_shared::{ffi::jolang_std::JOLANG_STD, ir::{block::Block, instructions::Instruction}};
use std::{cell::RefCell, collections::LinkedList};

pub struct LLVMRuntime {
    ctx : Context,
}

impl LLVMRuntime {
    fn get_int_type<'b>(&'b self, size : u64) -> Result<IntType<'b>> {
        Ok(match size {
            8 => self.ctx.i8_type(), 
            16 => self.ctx.i16_type(),
            32 => self.ctx.i32_type(),
            64 => self.ctx.i64_type(),
            128 => self.ctx.i128_type(),
            _ => {
                return Err(anyhow!("unsupported integer type : i{}", size))
            }
        })
    }
    
    fn load_externs<'b>(&'b self, table : &Vec<(String, u8, bool)>, module : &Module<'b>, builder : &Builder) -> Result<()>{
        for (name, argc, returns) in table {
            let std_sig = JOLANG_STD.iter()
                .filter(|x| x.0 == name)
                .next().map(|x| &x.1)
                .map_or_else(|| Err(anyhow!("unknown function : {}", name)), |x| Ok(x))?;
            let context : &Context = &self.ctx;
            if std_sig.returns() != *returns || std_sig.arg_count() != *argc {
                return Err(anyhow!("singnature of the function : \"{}\" is not the same between the runtime and the object", name));
            }
            let i64_type = context.i64_type();
            let args = vec![BasicMetadataTypeEnum::from(i64_type); *argc as usize];
            let sig = if *returns { 
                i64_type.fn_type(&args[..], false)
            } else { 
                context.void_type().fn_type(&args[..], false)
            };
            let fn_value = module.add_function(&name, sig, None);
            let block = context.append_basic_block(fn_value, "call");
            builder.position_at_end(block);
            let fn_ptr = unsafe {
                i64_type
                    .const_int(std_sig.get_pointer() as *const usize as u64, false)
                    .const_to_pointer(sig.ptr_type(AddressSpace::default()))
            };
            let ret = builder.build_indirect_call(sig, fn_ptr, &fn_value.get_params().iter().map(|x| BasicMetadataValueEnum::from(*x)).collect::<Vec<_>>()[..], "res")?;
            if let Some(val) = ret.try_as_basic_value().left() {
                builder.build_return(Some(&val))?;
            }else{
                builder.build_return(None)?;
            }
        }
        return Ok(())
    }

    pub fn gen_function(&self, blocks : &Vec<Block>, fn_value : FunctionValue, module : &Module, builder : &Builder) -> Result<()>{
        let i64_type = self.ctx.i64_type();

        let mut llvm_blocks : Vec<(Vec<PhiValue>, BasicBlock)> = Vec::new();
        // init blocks and args as phi
        for (i, blk) in blocks.iter().enumerate(){
            let llvm_blk = self.ctx.append_basic_block(fn_value, format!("B{}", i).as_str());
            builder.position_at_end(llvm_blk);
            let mut args = Vec::new();
            for arg in blk.args.iter() {
                let arg = builder.build_phi(self.get_int_type(*arg)?, "")?;
                args.push(arg);
            }
            llvm_blocks.push((args, llvm_blk));
        }

        // add instructions
        for (id, (blk,(args,llvm_blk))) in blocks.iter()
        .zip(llvm_blocks.iter())
        .enumerate(){
            builder.position_at_end(*llvm_blk);
            let mut stack = LinkedList::new();
            for a in args {
                stack.push_back(a.as_basic_value());
            }
            for i in &blk.instructions {
                match i  {
                    Instruction::Ret() => {
                        builder.build_return(None)?;
                    },
                    Instruction::Reti() => {
                        if let Some(value) = stack.pop_back() {
                            builder.build_return(Some(&value))?;
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building reti in B{}", id))
                        }
                    },
                    Instruction::Iconst(size, value) => {
                        let val = self.get_int_type(*size)?.const_int_arbitrary_precision(&[u64::from_le_bytes(value.to_le_bytes()[..8].try_into()?), u64::from_le_bytes(value.to_le_bytes()[8..].try_into()?)]);
                        stack.push_back(
                            val.into()
                        )
                    },
                    Instruction::Icast(size) => {
                        if let Some(value) = stack.pop_back() {
                            if value.into_int_value().get_type().get_bit_width() as u64 == *size {
                                stack.push_back(value);
                            }else if value.into_int_value().get_type().get_bit_width() as u64 > *size {
                                let t = self.get_int_type(*size)?;
                                let res = builder.build_int_truncate(value.into_int_value(), t, "res")?;
                                stack.push_back(res.into());
                            }else {
                                let t = match size {
                                    8 => self.ctx.i8_type(), 
                                    16 => self.ctx.i16_type(),
                                    32 => self.ctx.i32_type(),
                                    64 => self.ctx.i64_type(),
                                    128 => self.ctx.i128_type(),
                                    _ => {
                                        return Err(anyhow!("unsupported integer type : i{}", size))
                                    }
                                };
                                let res = builder.build_int_s_extend(value.into_int_value(), t, "res")?;
                                stack.push_back(res.into());
                            }
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building icast in B{}", id))
                        }
                    },
                    Instruction::Br(other_id) => {
                        if let Some(other_blk) = llvm_blocks.get(*other_id as usize) {
                            builder.build_unconditional_branch(other_blk.1)?;
                            for arg in other_blk.0.iter().rev().zip(stack.iter().rev()) {
                                arg.0.add_incoming(&[(arg.1, *llvm_blk)]);
                            }
                        }else {
                            return Err(anyhow!("tried to branch to a non existant block : {} in B{}", *other_id, id))
                        }
                    },
                    Instruction::Dup() => {
                        if let Some(value) = stack.back() {
                            stack.push_back(*value)
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building dup in B{}", id))
                        }
                    },
                    Instruction::Dupx(offset) => {
                        if let Some(value) = stack.iter().nth(*offset as usize) {
                            stack.push_back(*value)
                        }else {
                            return Err(anyhow!("tried to get a nonexistant value\nwhile building dupx in B{}", id))
                        }
                    },
                    Instruction::Swap() => {
                        if let Some(val1) = stack.pop_back(){
                            if let Some(val2) = stack.pop_back(){
                                stack.push_back(val2);
                                stack.push_back(val1);
                            }else {
                                return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap in B{}", id))
                            }
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap in B{}", id))
                            }
                    },
                    Instruction::Call(id) => {
                        if let Some(function) = module.get_functions()
                            .nth(*id as usize){
                            let mut args = Vec::new();
                            for _ in 0..function.count_params() {
                                if let Some(arg) = stack.pop_back(){
                                    args.push(arg.into());
                                }else {
                                    return Err(anyhow!("tried to call a function but the stack is too small"))
                                }
                            }
                            let value = builder.build_call(function, &args[..], "value")?;
                            if let Some(val) = value.try_as_basic_value().left() {
                                stack.push_back(val);
                            }
                        }else {
                            return Err(anyhow!("call of unregistered function : {}", id))
                        }
                    },
                    Instruction::Neg() => {
                        if let Some(value) = stack.pop_back() {
                            let result = builder.build_int_sub(
                                i64_type.const_zero(),
                                value.into_int_value() ,
                                "result")?;
                            stack.push_back(result.into())
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building neg in B{}", id))
                        }
                    },
                    Instruction::Add()
                        | Instruction::Sub()
                        | Instruction::Mul()
                        | Instruction::Div()
                        | Instruction::Eq()
                        | Instruction::Ne()
                        | Instruction::Gt()
                        | Instruction::Ge()
                        | Instruction::Le()
                        | Instruction::Lt()
                        | Instruction::Lsh()
                        | Instruction::Rsh()
                        => {
                        if let Some(val1) = stack.pop_back(){
                            if let Some(val2) = stack.pop_back(){
                                if val1.into_int_value().get_type().get_bit_width() != val2.into_int_value().get_type().get_bit_width() {
                                    return Err(anyhow!("mismached types i{}, i{}\n while building {}",
                                            val1.into_int_value().get_type().get_bit_width(),
                                            val2.into_int_value().get_type().get_bit_width(),
                                            match i {
                                                Instruction::Add() => "add",
                                                Instruction::Sub() => "sub",
                                                Instruction::Mul() => "mul",
                                                Instruction::Div() => "div",
                                                Instruction::Eq() => "eq",
                                                Instruction::Ne() => "ne",
                                                Instruction::Gt() => "gt",
                                                Instruction::Ge() => "ge",
                                                Instruction::Le() => "le",
                                                Instruction::Lt() => "lt",
                                                Instruction::Lsh() => "lsh",
                                                Instruction::Rsh() => "rsh",
                                                _ => unreachable!()
                                            }))
                                }
                                let result_type = val1.into_int_value().get_type();
                                let result = match i {
                                    Instruction::Add() => {
                                        builder.build_int_add(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                        "res")?
                                    },
                                    Instruction::Sub() => {
                                        builder.build_int_sub(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                        "res")?
                                    },
                                    Instruction::Mul() => {
                                        builder.build_int_mul(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                        "res")?
                                    },
                                    Instruction::Div() => {
                                        builder.build_int_signed_div(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                        "res")?
                                    },
                                    Instruction::Eq() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::EQ,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Ne() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::NE,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Gt() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SGT,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Ge() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SGE,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Le() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SLE,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Lt() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SLT,
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            result_type,
                                            "res")?
                                    },
                                    Instruction::Lsh() => {
                                        builder.build_left_shift(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                        "res")?
                                    },
                                    Instruction::Rsh() => {
                                        builder.build_right_shift(
                                            val2.into_int_value(),
                                            val1.into_int_value(),
                                            false,
                                        "res")?
                                    },
                                    _ => unreachable!()
                                };
                                stack.push_back(result.into());
                            }else {
                                return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap in B{}", id))
                            }
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap in B{}", id))
                        }
                    },
                    Instruction::Briz(id1, id2) => {
                        if let Some(blk1) = llvm_blocks.get(*id1 as usize) {
                            if let Some(blk2) = llvm_blocks.get(*id2 as usize){
                                if let Some(cond) = stack.pop_back() {
                                    let cond = builder.build_int_compare(
                                        inkwell::IntPredicate::EQ,
                                        cond.into_int_value(),
                                        i64_type.const_zero().into(),
                                        "cond")?;
                                    builder.build_conditional_branch(
                                        cond,
                                        blk1.1,
                                        blk2.1
                                    )?;
                                    for arg in blk1.0.iter().rev().zip(stack.iter().rev()) {
                                        arg.0.add_incoming(&[(arg.1, *llvm_blk)]);
                                    }
                                    for arg in blk2.0.iter().rev().zip(stack.iter().rev()) {
                                        arg.0.add_incoming(&[(arg.1, *llvm_blk)]);
                                    }
                                }else {
                                    return Err(anyhow!("tried to get a value from an empty stack\nwhile building briz in B{}", id))
                                }
                            }else {
                                return Err(anyhow!("tried to branch to a non existant block : {} in B{}", *id2, id))
                            }
                        }else {
                            return Err(anyhow!("tried to branch to a non existant block : {}, in B{}", *id1, id))
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Runtime for LLVMRuntime {
    fn new() -> Self {
        Self {
            ctx : Context::create()
        }
    }

    fn run(&mut self, object : jolang_shared::ir::IrObject) -> anyhow::Result<i64> {
        let module = self.ctx.create_module("jolang_main");
        let builder = self.ctx.create_builder();
        self.load_externs(&object.ext_fn, &module, &builder)?;
        let main_sig = self.ctx.i64_type().fn_type(&[], false);
        let main_value = module.add_function("main", main_sig, None);
        self.gen_function(&object.blocks, main_value, &module, &builder)?;

        module.print_to_stderr();

        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();

        unsafe {
            type MainFn = unsafe extern "C" fn() -> i64;
            let main_fn: JitFunction<MainFn> = execution_engine.get_function("main").unwrap();
            Ok(main_fn.call())
        }
    }
}

