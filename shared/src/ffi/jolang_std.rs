//! standard (builtin functions)

use rand::Rng;
use lazy_static::lazy_static;
use super::JolangExtern;

extern "C" fn print(value : i64) {
    println!("{}", value);
}

extern "C" fn input() -> i64 {
    print!("input: ");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read input from stdout");
    return line.trim().parse().expect("Failed to parse integer");
}

extern "C" fn pow(value : i64, exponent : i64) -> i64 {
    value.pow(exponent as u32)
}

extern "C" fn randint(min : i64, max : i64) -> i64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..max)
}

lazy_static! {
    pub static ref JOLANG_STD : [(&'static str, Box<dyn JolangExtern>);4] = [
        ("print", Box::new(print as extern "C" fn(i64))),
        ("input", Box::new(input as extern "C" fn() -> i64)),
        ("pow", Box::new(pow as extern "C" fn(i64, i64) -> i64)),
        ("randint", Box::new(randint as extern "C" fn(i64, i64) -> i64))
    ];
}
