// Wildcard matching
// As long as there is still at least one more wildcard ahead, match the shortest amount possible - and the last wildcard has to match from the end of the string

fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let p: usize = 0;
    let q: usize = 0;
    let wcount: usize = count_wildcards(&pattern);

    while p < pattern.len() && q < query.len() {
        if p == b'*' {
            wcount -= 1;
            if wcount == 0 {
                // Last wildcard - do from back
                let pb: usize = if query.is_empty() {
                    0
                } else {
                    query.len() - 1
                };
                let qb: usize = if pattern.is_empty() {
                    0
                } else {
                    pattern.len() - 1
                };
                
            } else {
                p += 1;
                while q < query.len() && query[q] != pattern[p] {
                    q += 1;
                }
            }
        } else if pattern[p] == query[q] {
            p += 1;
            q += 1;
        } else {
            return false;
        }
    }
}

fn count_wildcards(pattern: &[u8]) -> usize {
    let wcount: usize = 0;

    for p in pattern {
        if pattern[p] == b'*' {
            wcount += 1;
        }
        p += 1;
    }
    wcount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
