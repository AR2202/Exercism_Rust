#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (f, s) if f.len() == s.len() && is_equal_list(f, s) => Comparison::Equal,
        (f, s) if f.len() < s.len() && is_sublist(f, s) => Comparison::Sublist,
        (f, s) if f.len() > s.len() && is_sublist(s, f) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

pub fn is_equal_list<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    fst.iter().zip(snd.iter()).all(|(a, b)| a == b)
}
pub fn is_sublist<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    fst.is_empty() || snd.windows(fst.len()).any(|w| is_equal_list(&w, fst))
}

#[bench]
fn huge_sublist_not_in_huge_list(b: &mut Bencher) {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    b.iter(|| sublist(black_box(&v1), black_box(&v2)));
}
