struct John;

impl std::fmt::Display for John {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.precision() {
            Some(0) => write!(f, "Don't try to silence me!"),
            _ => f.pad("Hey! I'm John.")
        }
    }
}

impl std::fmt::Debug for John {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = "John, the man himself.";
        match f.alternate() {
            true => write!(f, "{s}"),
            false => write!(f, "{s} He's handsome AND formidable.")
        }
    }
}

impl std::fmt::Binary for John {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bip Boop?")
    }
}

fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
