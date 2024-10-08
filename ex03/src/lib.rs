#[allow(dead_code)]
fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    let mut longest: &[u32] = &[];
    let mut start: Option<usize> = None;
    let mut i: usize = 0;

    while i < haystack.len() {
        if needle.contains(&haystack[i]) {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(start_index) = start {
            if is_there(&haystack[start_index..i], needle) && i - start_index > longest.len() {
                longest = &haystack[start_index..i];
            }
            start = None;
        }
        i += 1;
    }
    longest
}

fn is_there(candidate: &[u32], needle: &[u32]) -> bool {
    for n in needle {
        if !candidate.contains(n) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[3]), &[3]);
    }
}
