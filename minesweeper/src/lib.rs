use std::char::from_digit;
use std::cmp::{max, min};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let new_board: Vec<String> = minefield
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let new_chars: String = x
                .chars()
                .enumerate()
                .map(|pair| match pair {
                    (_, '*') => '*',
                    (j, _) => match from_digit(neighbours_mines(i, j, minefield), 10).unwrap() {
                        '0' => ' ',
                        c => c,
                    },
                })
                .collect();
            new_chars
        })
        .collect();

    new_board
}

pub fn neighbours_mines(i: usize, j: usize, minefield: &[&str]) -> u32 {
    let i_start = max(i, 1) - 1;
    let j_start = max(j, 1) - 1;
    let vec: Vec<Vec<char>> = minefield[i_start..min(i + 2, minefield.len())]
        .iter()
        .map(|s| {
            let idxs: Vec<usize> = (j_start..j + 2).into_iter().collect();
            let chrs: Vec<char> = s
                .chars()
                .enumerate()
                .filter(|&(ind, _x)| idxs.contains(&ind))
                .map(|(_idx, x)| x)
                .collect();
            chrs
        })
        .collect();
    let flattened = vec.concat();
    let mines: Vec<&char> = flattened.iter().filter(|&c| *c == '*').collect();
    mines.len() as u32
}
