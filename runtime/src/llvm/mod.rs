use inkwell::{context::Context, module::Module, builder::Builder};

struct LLVMRuntime<'a> {
    ctx : Context,
    module : Module<'a>,
    builder : Builder<'a>
}


