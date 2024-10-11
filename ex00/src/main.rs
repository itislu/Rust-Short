use std::io::Write;

fn main() {
    for i in 1..=10 {
        if let Err(_) = writeln!(std::io::stdout(), "{i}") {
            break
        }
    }
}
