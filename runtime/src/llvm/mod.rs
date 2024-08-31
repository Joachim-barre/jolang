use anyhow::{anyhow, Result};
use inkwell::{builder::Builder, context::Context, execution_engine::JitFunction, module::Module, types::{BasicMetadataTypeEnum, BasicType}, values::{BasicMetadataValueEnum, BasicValue, FunctionValue, PhiValue}, AddressSpace, OptimizationLevel, basic_block::BasicBlock};
use crate::Runtime;
use jolang_shared::{ffi::jolang_std::JOLANG_STD, ir::block::Block};
use std::cell::RefCell;

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

    pub fn gen_function(&self, blocks : &Vec<RefCell<Block>>, fn_value : FunctionValue, builder : &Builder) -> Result<()>{
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
        .zip(llvm_blocks){
            builder.position_at_end(llvm_blk);
            for i in &blk.instructions {
                match i  {
                    _ => todo!()
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
        self.gen_function(&object.blocks, main_value, &builder)?;

        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();

        unsafe {
            type MainFn = unsafe extern "C" fn() -> i64;
            let main_fn: JitFunction<MainFn> = execution_engine.get_function("main").unwrap();
            Ok(main_fn.call())
        }
    }
}

