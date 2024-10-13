#[allow(dead_code)]
fn deduplicate(list: &mut Vec<i32>) {
    let mut i = 0;

    while i + 1 < list.len() {
        let mut j = i + 1;
        while j < list.len() {
            if list[j] == list[i] {
                list.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn empty_vector() {
        let mut v: Vec<i32> = vec![];
        deduplicate(&mut v);
        assert_eq!(v, []);
    }

    #[test]
    fn single_element() {
        let mut v = vec![1];
        deduplicate(&mut v);
        assert_eq!(v, [1]);
    }

    #[test]
    fn no_duplicates() {
        let mut v = vec![1, 2, 3, 4];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn all_duplicates() {
        let mut v = vec![1, 1, 1, 1];
        deduplicate(&mut v);
        assert_eq!(v, [1]);
    }

    #[test]
    fn mixed_order() {
        let mut v = vec![2, 2, 1, 1, 3, 3, 2, 1];
        deduplicate(&mut v);
        assert_eq!(v, [2, 1, 3]);
    }

    #[test]
    fn large_numbers() {
        let mut v = vec![1000000, 1000000, 999999, 999999, 1000000];
        deduplicate(&mut v);
        assert_eq!(v, [1000000, 999999]);
    }

    #[test]
    fn negative_numbers() {
        let mut v = vec![-1, -2, -2, -3, -1, -4, -3];
        deduplicate(&mut v);
        assert_eq!(v, [-1, -2, -3, -4]);
    }

    #[test]
    fn mixed_positive_and_negative() {
        let mut v = vec![1, -1, 2, -2, 1, -1, 2, -2];
        deduplicate(&mut v);
        assert_eq!(v, [1, -1, 2, -2]);
    }
}
