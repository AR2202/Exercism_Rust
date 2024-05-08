/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let iter = code.chars().filter(|&c| c != ' ');
    iter.clone().count() > 1
        && iter.clone().all(|c| c.is_digit(10))
        && iter
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
