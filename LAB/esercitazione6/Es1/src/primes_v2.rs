use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut primes = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    for i in 0..n_threads {
        let mut p = primes.clone();
        threads.push(thread::spawn(move || {
            let mut count = 2 + i;
            loop {
                let mut local_primes = p.lock().unwrap();
                if count > limit {
                    break;
                }
                if is_prime(count) {
                    local_primes.push(count);
                }
                count += n_threads;
            }
        }))
    }

    for t in threads {
        t.join().unwrap();
    }
    let mut result = primes.lock().unwrap().clone();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_find_primes_10() {
        let start = Instant::now();
        let mut result = find_primes(10, 2);
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7]);
        println!("Tempo test_find_primes_10: {:?}", start.elapsed());
    }

    #[test]
    fn test_find_primes_20_single_thread() {
        let start = Instant::now();
        let mut result = find_primes(20, 1);
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        println!("Tempo test_find_primes_20_single_thread: {:?}", start.elapsed());
    }

    #[test]
    fn test_find_primes_20_multi_thread() {
        let start = Instant::now();
        let mut result = find_primes(20, 4);
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        println!("Tempo test_find_primes_20_multi_thread: {:?}", start.elapsed());
    }

    #[test]
    fn test_find_primes_zero() {
        let start = Instant::now();
        let result = find_primes(0, 2);
        assert!(result.is_empty());
        println!("Tempo test_find_primes_zero: {:?}", start.elapsed());
    }

    #[test]
    fn test_find_primes_one() {
        let start = Instant::now();
        let result = find_primes(1, 2);
        assert!(result.is_empty());
        println!("Tempo test_find_primes_one: {:?}", start.elapsed());
    }
}

fn main() {
    println!("Hello, world!");
}