fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    println!("{}", min(2147483647, -2147483648));
}
