pub mod linux;
use core::panic;
use crate::Runtime;

pub trait Platform {
    fn new() -> Self;
    fn default_runtime(&self) -> Box<dyn Runtime>;
}

pub struct UnsupportedPlatform {}

impl Platform for UnsupportedPlatform {
    fn new() -> Self {
        panic!("unsupported platform : {}", current_platform::CURRENT_PLATFORM)
    }
    
    fn default_runtime(&self) -> Box<dyn Runtime> {
        unimplemented!()
    }
}

#[cfg(target_os = "linux")]
pub type CurrentPlatform = linux::LinuxPlatform;

#[cfg(not(any(target_os = "linux")))]
pub type CurrentPlatform = UnsupportedPlatform;
