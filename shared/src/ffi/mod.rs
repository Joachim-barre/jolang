use crate::ir::signature::Signature;

pub mod jolang_std;

pub trait JolangExtern : Sync {
    fn signature(&self) -> Signature;
    unsafe fn get_pointer(&self) -> u64;
}

impl JolangExtern for extern "C" fn(i64) {
    fn signature(&self) -> Signature {
        Signature { 
            ret: String::from("void"),
            args: vec![String::from("i64")]
        }
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}

impl JolangExtern for extern "C" fn() -> i64 {
    fn signature(&self) -> Signature {
        Signature { 
            ret: String::from("i64"),
            args: Vec::new() 
        }
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}

impl JolangExtern for extern "C" fn(i64, i64) -> i64 {
    fn signature(&self) -> Signature {
        Signature {
            ret: String::from("i64"),
            args: vec![
                String::from("i64"),
                String::from("i64")
            ]
        }
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}
