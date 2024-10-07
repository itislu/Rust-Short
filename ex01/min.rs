fn min(a: i32, b: i32) -> i32 {
    let min: i32;

    if a < b {
        min = a;
    } else {
        min = b;
    }
    min
}

fn main() {
    println!("{}", min(2, 4));
}
