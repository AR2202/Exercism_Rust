use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hm: Arc<RwLock<HashMap<char, usize>>> = Arc::new(RwLock::new(HashMap::new()));
    if input.is_empty() {
        return (*hm.read().unwrap()).clone();
    }
    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    for chunk in input.chunks(chunk_size) {
        hm = Arc::clone(&hm);
        rayon::scope(|sc| {
            sc.spawn(|_| {
                let mut hm1 = hm.write().unwrap();
                for s in chunk.iter() {
                    for ch in s.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                        *hm1.entry(ch).or_insert(0) += 1;
                    }
                }
            });
        });
    }
    let result = (*hm.read().unwrap()).clone();
    result
}
