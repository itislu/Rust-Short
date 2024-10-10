fn choose<T>(values: &[T]) -> &T {
    &values[ftkit::random_number(0..(values.len() as i32)) as usize]
}

fn main() {
    let values = [1, 2, 3, 4, 5];
    println!("{}", choose(&values));
}
