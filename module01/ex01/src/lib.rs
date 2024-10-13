fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 2;
        let result;
        {
            let b = 4;
            result = min(&a, &b);
            assert_eq!(*result, 2);
        }
    }
}
