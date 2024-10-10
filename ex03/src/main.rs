use std::fmt;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for u32 {
    fn forty_two() -> Self {
        42
    }
}

impl FortyTwo for i32 {
    fn forty_two() -> Self {
        -42
    }
}

impl FortyTwo for String {
    fn forty_two() -> Self {
        String::from("fortytwo")
    }
}

fn print_forty_two<T: fmt::Debug + FortyTwo>() {
    let t: T = FortyTwo::forty_two();
    println!("{:?}", t);
}

fn main() {
    print_forty_two::<u32>();
    print_forty_two::<String>();
    print_forty_two::<i32>();
}
