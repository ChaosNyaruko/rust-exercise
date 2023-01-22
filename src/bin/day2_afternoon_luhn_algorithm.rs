#![allow(unused_variables, dead_code)]

/// The Luhn algorithm is used to validate credit card numbers.
/// The algorithm takes a string as input and does the following to validate the credit card number:
/// Ignore all spaces. Reject number with less than two digits.
/// Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.
/// After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.
/// Sum all the undoubled and doubled digits.
///The credit card number is valid if the sum is ends with 0.
pub fn luhn(cc_number: &str) -> bool {
    // let cc_number = cc_number.to_string();
    println!("{:?}", cc_number);
    let without_space = cc_number.chars().filter(|x| x.is_digit(10));
    let rev = without_space.rev();

    let mut total_sum = 0;

    let mut valid = false;
    for (i, c) in rev.enumerate() {
        if i >= 1 {
            valid = true;
        }
        println!("i = {}, {:?}", i, c);
        if i % 2 == 1 {
            // double it
            let mut num = c.to_digit(10).unwrap() * 2;
            println!("{}", num);
            // sum the digits
            let mut s_num = 0;
            while num > 0 {
                s_num += num % 10;
                num /= 10;
            }
            total_sum += s_num;
        } else {
            total_sum += c.to_digit(10).unwrap();
        }
    }
    valid && total_sum % 10 == 0
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
fn main() {
    println!("{}", luhn("0"));
}
