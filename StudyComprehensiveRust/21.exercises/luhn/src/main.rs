// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    if cc_number
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .count()
        < 2
    {
        return false;
    }

    if cc_number
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .any(|c| !c.is_digit(10))
    {
        return false;
    }

    cc_number
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| {
            if i % 2 == 0 {
                acc + n.to_digit(10).unwrap()
            } else {
                let mut n = n.to_digit(10).unwrap();
                n *= 2;
                acc + n / 10 + n % 10
            }
        })
        % 10
        == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
