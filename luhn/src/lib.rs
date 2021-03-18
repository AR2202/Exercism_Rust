/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
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
