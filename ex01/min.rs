fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    let mut a: i32;
    let b = 0;

    a = 2147483647;
    println!("{}", min(a, b));
    a += 1;
    println!("{}", min(a, b));
}
