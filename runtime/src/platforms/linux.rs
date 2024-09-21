use super::Platform;
use crate::Runtime;
#[cfg(feature = "llvm")]
use crate::llvm::LLVMRuntime;

pub struct LinuxPlatform  {}

impl Platform for LinuxPlatform {
    fn new() -> Self {
        Self {}
    }


    #[cfg(feature = "llvm")]
    fn default_runtime(&self) -> Box<dyn crate::Runtime> {
        Box::new(LLVMRuntime::new())
    }

    #[cfg(not(all(feature = "llvm")))]
    fn default_runtime(&self) -> Box<dyn crate::Runtime> {
        panic!("no compatible runtimes found for platform linux\ntry to activate a compatible runtime feature") 
    }
}
