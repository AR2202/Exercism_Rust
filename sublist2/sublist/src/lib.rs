#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() && is_equal_list(first_list, second_list) {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if first_list.len() > second_list.len() && is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn is_equal_list<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    fst.iter().zip(snd.iter()).all(|(a, b)| a == b)
}
pub fn is_sublist<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    match fst.is_empty() {
        true => true,
        false => snd.windows(fst.len()).any(|w| is_equal_list(&w, fst)),
    }
}
