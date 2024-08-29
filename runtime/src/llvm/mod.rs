use inkwell::{context::Context, module::Module, builder::Builder, OptimizationLevel, execution_engine::JitFunction};
use crate::Runtime;

pub struct LLVMRuntime<'ctx> {
    ctx : Context,
    module : Option<Module<'ctx>>,
}

impl<'a> Runtime for LLVMRuntime<'a> {
    fn new() -> Self {
        Self {
            ctx : Context::create(),
            module : None
        }
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

