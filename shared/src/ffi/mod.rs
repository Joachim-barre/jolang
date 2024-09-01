pub mod jolang_std;

pub trait JolangExtern : Sync {
    fn returns(&self) -> bool;
    fn arg_count(&self) -> u8;
    unsafe fn get_pointer(&self) -> u64;
}

impl JolangExtern for extern "C" fn(i64) {
    fn returns(&self) -> bool {
        false
    }

    fn arg_count(&self) -> u8 {
        1
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}

impl JolangExtern for extern "C" fn() -> i64 {
    fn returns(&self) -> bool {
        true
    }

    fn arg_count(&self) -> u8 {
        0
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}

impl JolangExtern for extern "C" fn(i64, i64) -> i64 {
    fn returns(&self) -> bool {
        true
    }

    fn arg_count(&self) -> u8 {
        2
    }

    unsafe fn get_pointer(&self) -> u64 {
        std::mem::transmute(*self)
    }
}
