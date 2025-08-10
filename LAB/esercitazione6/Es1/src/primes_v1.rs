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
    let mut count = Arc::new(Mutex::new(2 as u64));

    for i in 0..n_threads {
        let mut p = primes.clone();
        let mut cnt = count.clone();
        threads.push(thread::spawn(move || {
            loop{
                let mut local_primes = p.lock().unwrap();
                let mut c = cnt.lock().unwrap();
                if *c > limit {
                    break;
                }
                if is_prime(*c) {
                    local_primes.push(*c);
                }
                *c += 1;
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_primes_10() {
        let mut result = find_primes(10, 2);
        let duration = Instant::now();
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7]);
        println!("Time taken: {:?}", duration.elapsed());
    }

    #[test]
    fn test_find_primes_20_single_thread() {
        let mut result = find_primes(20, 1);
        let duration = Instant::now();
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        println!("Time taken: {:?}", duration.elapsed());
    }

    #[test]
    fn test_find_primes_20_multi_thread() {
        let mut result = find_primes(20, 4);
        let duration = Instant::now();
        result.sort();
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        println!("Time taken: {:?}", duration.elapsed());
    }

    #[test]
    fn test_find_primes_zero() {
        let result = find_primes(0, 2);
        let duration = Instant::now();
        assert!(result.is_empty());
        println!("Time taken: {:?}", duration.elapsed());
    }

    #[test]
    fn test_find_primes_one() {
        let result = find_primes(1, 2);
        let duration = Instant::now();
        assert!(result.is_empty());
        println!("Time taken: {:?}", duration.elapsed());
    }
}

