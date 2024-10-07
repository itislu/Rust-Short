fn do_overflow(n: u8) -> u8 {
    n + (u8::MAX - n) + 1
}

fn main() {
    println!("255u8 + 1u8 == {}", do_overflow(255));
}
