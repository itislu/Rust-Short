fn add(a: &i32, b: i32) -> i32 {
    a + b
}

fn add_assign(a: &mut i32, b: i32) {
    *a += b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(&2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_assign() {
        let mut a = 10;
        add_assign(&mut a, 2);
        assert_eq!(a, 12);
    }
}
