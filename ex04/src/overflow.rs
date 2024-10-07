fn main() {
    let n = 255u8;

    println!("n: {n}");
    if n + 1u8 == 0 {
        println!("overflow");
    } else {
        println!("no overflow");
    }
}
