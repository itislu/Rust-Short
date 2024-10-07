fn min(a: i32, b: i32) -> i32 {
    let min = if a < b { a } else { b };
    min
}

fn main() {
    println!("{}", min(2, 4));
}
