#[no_mangle]
pub extern "C" fn print_int(n : i64) {
    println!("{}", n);
}

#[used] pub static PRINT_INT: extern "C" fn (i64) = print_int;
