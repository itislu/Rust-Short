#[allow(dead_code)]
fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let mut carry: u8 = 0;

    for (n, m) in a.iter().rev().zip(b.iter().rev()) {
        carry = add_vec(&mut vec, *n, *m, carry);
    }
    match (a.len() < b.len(), b.len() < a.len()) {
        (true, _) => carry = add_remainder(&mut vec, &b[0..b.len() - a.len()], carry),
        (_, true) => carry = add_remainder(&mut vec, &a[0..a.len() - b.len()], carry),
        _ => (),
    }
    add_vec(&mut vec, b'0', b'0', carry);

    reverse_no_lead_zeros(&vec)
}

fn add_vec(vec: &mut Vec<u8>, mut a: u8, mut b: u8, old_carry: u8) -> u8 {
    let mut res;
    let mut new_carry = 0u8;

    if !a.is_ascii_digit() || !b.is_ascii_digit() {
        panic!();
    }
    a -= b'0';
    b -= b'0';
    res = a + b + old_carry;
    if res > 9 {
        res -= 10;
        new_carry = 1;
    }
    vec.push(res + b'0');
    new_carry
}

fn add_remainder(vec: &mut Vec<u8>, remainder: &[u8], mut carry: u8) -> u8 {
    let mut i = remainder.len();
    loop {
        i -= 1;
        carry = add_vec(vec, remainder[i], b'0', carry);
        if i == 0 {
            break;
        }
    }
    carry
}

// Remove leading zeros and reverse the vector
fn reverse_no_lead_zeros(vec: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut is_lead_zero = true;

    for n in vec.iter().rev() {
        if is_lead_zero && *n != b'0' {
            is_lead_zero = false;
        }
        if !is_lead_zero {
            res.push(*n);
        }
    }
    if res.is_empty() {
        res.push(b'0');
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(big_add(b"2", b"4"), b"6");
        assert_eq!(big_add(b"0010", b"0200"), b"210");
    }

    #[test]
    fn different_lengths() {
        assert_eq!(big_add(b"123", b"45678"), b"45801");
        assert_eq!(big_add(b"45678", b"123"), b"45801");
    }

    #[test]
    fn leading_zeros() {
        assert_eq!(big_add(b"000123", b"000456"), b"579");
        assert_eq!(big_add(b"123", b"000456"), b"579");
    }

    #[test]
    fn carry_over() {
        assert_eq!(big_add(b"999", b"1"), b"1000");
        assert_eq!(big_add(b"9999", b"1"), b"10000");
    }

    #[test]
    fn large_numbers() {
        assert_eq!(
            big_add(b"12345678901234567890", b"98765432109876543210"),
            b"111111111011111111100"
        );
        assert_eq!(
            big_add(b"99999999999999999999", b"1"),
            b"100000000000000000000"
        );
    }

    #[test]
    fn mixed_digits() {
        assert_eq!(big_add(b"12345", b"678"), b"13023");
        assert_eq!(big_add(b"678", b"12345"), b"13023");
    }

    #[test]
    fn zero_addition() {
        assert_eq!(big_add(b"0", b"0"), b"0");
        assert_eq!(big_add(b"12345", b"0"), b"12345");
        assert_eq!(big_add(b"0", b"12345"), b"12345");
    }

    #[test]
    fn single_digit_addition() {
        assert_eq!(big_add(b"5", b"5"), b"10");
        assert_eq!(big_add(b"9", b"1"), b"10");
    }
}
