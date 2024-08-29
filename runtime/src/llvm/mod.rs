use inkwell::{context::Context, module::Module, builder::Builder};

pub struct LLVMRuntime<'ctx> {
    ctx : Context,
    module : Option<Module<'ctx>>,
}

impl<'a> LLVMRuntime<'a> {
    pub fn new() -> Self {
        Self {
            ctx : Context::create(),
            module : None
        }
    }
}

