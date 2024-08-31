use anyhow::{anyhow, Result};
use inkwell::{builder::Builder, context::Context, execution_engine::JitFunction, module::Module, types::{BasicMetadataTypeEnum, BasicType}, values::{BasicMetadataValueEnum, BasicValue}, AddressSpace, OptimizationLevel};
use crate::Runtime;
use jolang_shared::ffi::jolang_std::JOLANG_STD;

pub struct LLVMRuntime<'ctx> {
    ctx : Context,
    module : Option<Module<'ctx>>,
}

impl<'a> LLVMRuntime<'a> {
    fn load_externs(&self, table : &Vec<(String, u8, bool)>, module : &Module, builder : &Builder) -> Result<()>{
        for (name, argc, returns) in table {
            let std_sig = JOLANG_STD.iter()
                .filter(|x| x.0 == name)
                .next().map(|x| &x.1)
                .map_or_else(|| Err(anyhow!("unknown function : {}", name)), |x| Ok(x))?;
            let context : &Context = unsafe { std::mem::transmute(&self.ctx) };
            if std_sig.returns() != *returns || std_sig.arg_count() != *argc {
                return Err(anyhow!("singnature of the function {} is not the same between the runtime and the object", name));
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
}

impl<'a> Runtime for LLVMRuntime<'a> {
    fn new() -> Self {
        Self {
            ctx : Context::create(),
            module : None
        }
    }

    fn load(&mut self, object : jolang_shared::ir::IrObject) -> anyhow::Result<()> {
        if self.module.is_some() {
            return Err(anyhow!("module is already initialized"))
        }
        unsafe {
            self.module = Some(std::mem::transmute(self.ctx.create_module("jolang_main")));
        }
        let module = self.module.as_ref().unwrap();
        let builder = self.ctx.create_builder();
        self.load_externs(&object.ext_fn, module, &builder);
        todo!()
    }

    fn run(&mut self) -> i64 {
        if self.module.is_none() {
            panic!("tried to run an unitialized module");
        }
        let module = self.module.as_mut().unwrap();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();

        unsafe {
            type MainFn = unsafe extern "C" fn() -> i64;
            let main_fn: JitFunction<MainFn> = execution_engine.get_function("main").unwrap();
            main_fn.call()
        }
    }
}

