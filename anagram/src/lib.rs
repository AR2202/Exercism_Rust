use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let mut sorted: Vec<char> = lowercase_word.chars().collect();
    sorted.sort_unstable();

    let answers: HashSet<&str> = possible_anagrams
        .iter()
        .cloned()
        .filter(|s| {
            let s_lowercase = s.to_lowercase();
            s.len() == word.len()
                && s_lowercase != lowercase_word
                && sort_chars(&s_lowercase) == sorted
        })
        .collect();
    answers
}

fn sort_chars(s: &str) -> Vec<char> {
    let mut lowercase_chars: Vec<char> = s.chars().collect();
    lowercase_chars.sort_unstable();
    lowercase_chars
}
