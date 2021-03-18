pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut p = 2;
    primes.push(p);
    while primes.len() < ((n + 1) as usize) {
        p += 1;
        if primes.iter().all(|prime| p % prime != 0) {
            primes.push(p);
        }
    }
    p
}
