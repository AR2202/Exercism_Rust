#![feature(test)]
extern crate test;
use test::Bencher;
/// Check a Luhn checksum.
pub fn is_valid1(code: &str) -> bool {
    let vec: Vec<char> = code.chars().filter(|&c| c != ' ').collect();
    vec.len() > 1
        && vec.iter().all(|c| c.is_digit(10))
        && vec
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| match (i, c.to_digit(10).unwrap()) {
                (i, d) if i % 2 == 0 => acc + d,
                (_i, d) if d * 2 > 9 => acc + 2 * d - 9,
                (_i, d) => acc + 2 * d,
            })
            % 10
            == 0
}
pub fn is_valid2(code: &str) -> bool {
    code.chars().filter(|&c| c != ' ').count() > 1
        && code.chars().filter(|&c| c != ' ').all(|c| c.is_digit(10))
        && code
            .chars()
            .filter(|&c| c != ' ')
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| match (i, c.to_digit(10).unwrap()) {
                (i, d) if i % 2 == 0 => acc + d,
                (_i, d) if d * 2 > 9 => acc + 2 * d - 9,
                (_i, d) => acc + 2 * d,
            })
            % 10
            == 0
}
fn process_valid_case(number: &str, is_luhn_expected: bool) {
    assert_eq!(is_valid2(number), is_luhn_expected);
}

#[bench]

/// valid number with an odd number of spaces
fn test_valid_number_with_an_odd_number_of_spaces(b: &mut Bencher) {
    b.iter(|| process_valid_case("234 567 891 234", true));
}
