use anyhow::{anyhow, Result};
use inkwell::{builder::Builder, context::Context, execution_engine::JitFunction, module::Module, types::{BasicMetadataTypeEnum, BasicType}, values::{BasicMetadataValueEnum, BasicValue, FunctionValue, PhiValue}, AddressSpace, OptimizationLevel, basic_block::BasicBlock};
use crate::Runtime;
use jolang_shared::{ffi::jolang_std::JOLANG_STD, ir::{block::Block, instructions::Instruction}};
use std::{cell::RefCell, collections::LinkedList};

pub struct LLVMRuntime {
    ctx : Context,
}

impl LLVMRuntime {
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

    pub fn gen_function(&self, blocks : &Vec<RefCell<Block>>, fn_value : FunctionValue, module : &Module, builder : &Builder) -> Result<()>{
        let i64_type = self.ctx.i64_type();

        let blocks = blocks.iter().map(|x| x.take()).collect::<Vec<_>>();
        let mut llvm_blocks : Vec<(Vec<PhiValue>, BasicBlock)> = Vec::new();
        // init blocks and args as phi
        for blk in blocks.iter(){
            let llvm_blk = self.ctx.append_basic_block(fn_value, "");
            builder.position_at_end(llvm_blk);
            let mut args = Vec::new();
            for i in 0..blk.argc {
                let arg = builder.build_phi(i64_type, "")?;
                args.push(arg);
            }
            llvm_blocks.push((args, llvm_blk));
        }

        // add instructions
        for (blk,(args,llvm_blk)) in blocks.iter()
        .zip(llvm_blocks.iter()){
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
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building reti"))
                        }
                    },
                    Instruction::Iconst(value) => {
                        stack.push_back(
                            i64_type.const_int(u64::from_le_bytes(value.to_le_bytes())
                            , false).into()
                        )
                    },
                    Instruction::Br(id) => {
                        if let Some(other_blk) = llvm_blocks.get(*id as usize) {
                            builder.build_unconditional_branch(other_blk.1)?;
                            for arg in other_blk.0.iter().rev().zip(stack.iter().rev()) {
                                arg.0.add_incoming(&[(arg.1, *llvm_blk)]);
                            }
                        }else {
                            return Err(anyhow!("tried to branch to a non existant block : {}", *id))
                        }
                    },
                    Instruction::Dup() => {
                        if let Some(value) = stack.back() {
                            stack.push_back(*value)
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building dup"))
                        }
                    },
                    Instruction::Dupx(offset) => {
                        if let Some(value) = stack.iter().nth(*offset as usize) {
                            stack.push_back(*value)
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building dupx"))
                        }
                    },
                    Instruction::Swap() => {
                        if let Some(val1) = stack.pop_back(){
                            if let Some(val2) = stack.pop_back(){
                                stack.push_back(val2);
                                stack.push_back(val1);
                            }else {
                                return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap"))
                            }
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap"))
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
                        if let Some(value) = stack.back() {
                            let result = builder.build_int_sub(
                                i64_type.const_zero(),
                                value.into_int_value() ,
                                "result")?;
                            stack.push_back(result.into())
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building neg"))
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
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
                                            "res")?
                                    },
                                    Instruction::Ne() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::NE,
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
                                            "res")?
                                    },
                                    Instruction::Gt() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SGT,
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
                                            "res")?
                                    },
                                    Instruction::Ge() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SGE,
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
                                            "res")?
                                    },
                                    Instruction::Le() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SLE,
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
                                            "res")?
                                    },
                                    Instruction::Lt() => {
                                        let cmp = builder.build_int_compare(
                                            inkwell::IntPredicate::SLT,
                                            val1.into_int_value(),
                                            val2.into_int_value(),
                                            "cmp")?;
                                        builder.build_int_z_extend(
                                            cmp,
                                            i64_type,
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
                                return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap"))
                            }
                        }else {
                            return Err(anyhow!("tried to get a value from an empty stack\nwhile building swap"))
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
                                    return Err(anyhow!("tried to get a value from an empty stack\nwhile building briz"))
                                }
                            }else {
                                return Err(anyhow!("tried to branch to a non existant block : {}", *id2))
                            }
                        }else {
                            return Err(anyhow!("tried to branch to a non existant block : {}", *id1))
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

        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();

        unsafe {
            type MainFn = unsafe extern "C" fn() -> i64;
            let main_fn: JitFunction<MainFn> = execution_engine.get_function("main").unwrap();
            Ok(main_fn.call())
        }
    }
}

