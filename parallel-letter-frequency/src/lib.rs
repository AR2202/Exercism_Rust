use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    input
        .par_chunks(chunk_size)
        .map(|chunk| {
            chunk
                .join("")
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .fold(HashMap::new(), |mut acc, ch| {
                    *acc.entry(ch).or_insert(0) += 1;
                    acc
                })
        })
        .reduce(|| HashMap::new(), merge_hashmaps)
}

fn merge_hashmaps(
    mut hm1: HashMap<char, usize>,
    hm2: HashMap<char, usize>,
) -> HashMap<char, usize> {
    for (k, v) in hm2.iter() {
        *hm1.entry(*k).or_insert(0) += v;
    }
    hm1
}
