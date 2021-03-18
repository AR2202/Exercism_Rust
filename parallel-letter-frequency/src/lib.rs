use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        let res: HashMap<char, usize> = HashMap::new();
        return res;
    }
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    let hm = input
        .par_chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .map(|s| {
                    s.to_lowercase().chars().filter(|c| c.is_alphabetic()).fold(
                        HashMap::new(),
                        |mut acc, ch| {
                            *acc.entry(ch).or_insert(0) += 1;
                            acc
                        },
                    )
                })
                .fold(HashMap::new(), |acc, hm2| merge_hashmaps(acc, hm2))
        })
        .reduce(|| HashMap::new(), |acc, hm2| merge_hashmaps(acc, hm2));

    hm
}

pub fn frequency2(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hm: Arc<Mutex<Option<HashMap<char, usize>>>> =
        Arc::new(Mutex::new(Some(HashMap::new())));
    if input.is_empty() {
        return hm.lock().unwrap().take().unwrap();
    }
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    for chunk in input.chunks(chunk_size) {
        hm = Arc::clone(&hm);
        rayon::scope(|sc| {
            sc.spawn(|_| {
                for s in chunk.iter() {
                    for ch in s.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                        *hm.lock().unwrap().take().unwrap().entry(ch).or_insert(0) += 1;
                    }
                }
            });
        });
    }
    let res = hm.lock().unwrap().take().unwrap();
    res
}

fn merge_hashmaps(
    mut hm1: HashMap<char, usize>,
    hm2: HashMap<char, usize>,
) -> HashMap<char, usize> {
    for (k, v) in hm2.iter() {
        let v_old = hm1.entry(*k).or_insert(0);
        *v_old += v;
    }
    hm1
}
