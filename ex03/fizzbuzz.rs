fn main() {
	for n in 1..100 {
		println!("{}",
			match (n % 3, n % 5, n % 11) {
				(0, 0, _) => "fizzbuzz".to_string(),
				(0, _, _) => "fizz".to_string(),
				(_, 0, _) => "buzz".to_string(),
				(_, _, 3) => "FIZZ".to_string(),
				(_, _, 5) => "BUZZ".to_string(),
				(_, _, _) => n.to_string()
			}
		);
	}
}
