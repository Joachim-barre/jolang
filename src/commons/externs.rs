//! standard (builtin functions)

mod jolang_std {
    use rand::Rng;

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

    static as_externs : [(&'static str, &'static (dyn super::JolangExtern + 'static));4] = {
        ("print", print),
    };
}

pub trait JolangExtern {
    fn returns() -> bool;
    fn arg_count() -> u8;
    unsafe fn get_pointer(self) -> u64;
}

impl JolangExtern for extern "C" fn(i64) {
    fn returns() -> bool {
        false
    }

    fn arg_count() -> u8 {
        1
    }

    unsafe fn get_pointer(self) -> u64 {
        std::mem::transmute(self)
    }
}

impl JolangExtern for extern "C" fn() -> i64 {
    fn returns() -> bool {
        false
    }

    fn arg_count() -> u8 {
        1
    }

    unsafe fn get_pointer(self) -> u64 {
        std::mem::transmute(self)
    }
}

impl JolangExtern for extern "C" fn(i64, i64) -> i64 {
    fn returns() -> bool {
        false
    }

    fn arg_count() -> u8 {
        1
    }

    unsafe fn get_pointer(self) -> u64 {
        std::mem::transmute(self)
    }
}
