use inkwell::{context::Context, module::Module, builder::Builder};
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
}

