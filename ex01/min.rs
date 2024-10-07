fn min(a: i32, b: i32) -> i32 {
    let min = if a < b { a } else { b };
    min
}

fn main() {
    let mut a: i32;
    let b = 0;

    a = 2147483647;
    println!("{}", min(a, b));
    a += 1;
    println!("{}", min(a, b));
}
