fn print_bytes(s: &str) {
    let s:String = s.to_string();
    for char in s.as_bytes().iter() {
        println!("{}", char);
    }
}

// fn main() {
// 	print_bytes("ssss");
// }
