use std::{sync::Arc, thread, time::{Duration, Instant}};

use crate::limiter::ExecutionLimiter;


pub mod limiter {
    use std::sync::{Arc, Condvar, Mutex};

    pub struct ExecutionLimiter {
        counter: Arc<(Mutex<usize>, Condvar)>,
        threshold: usize,
    }

    impl ExecutionLimiter {
        pub fn new(n: usize) -> Self {
            Self {
                counter: Arc::new((Mutex::new(0), Condvar::new())),
                threshold: n,
            }
        }

        pub fn execute<F, R: Send + Sync + 'static>(&self, f: F) -> Result<R, String> where F: Fn() -> Result<R, ()> + Send + Sync + 'static, 
        R: Send + Sync + 'static 
        {
            let (lock, condvar) = &*self.counter;
            let mut cnt = lock.lock().unwrap();

            cnt = condvar.wait_while(cnt, |c| *c >= self.threshold).unwrap();
            *cnt += 1;

            match f() {
                Ok(r) => {
                    *cnt -= 1;
                    condvar.notify_one();
                    return Ok(r);
                },
                Err(_) => {
                    *cnt -= 1;
                    condvar.notify_one();
                    return Err(format!("Error! Function didn't work"));
                },
            }
        }
    }
}


fn main() {
    let limiter = Arc::new(ExecutionLimiter::new(4)); // Max 2 esecuzioni concorrenti
    let mut handles = vec![];

    for i in 0..10 {
        let limiter = limiter.clone();

        let handle = thread::spawn(move || {
            let start = Instant::now();

            let result = limiter.execute(move || {
                println!("[{}] Inizio operazione", i);
                thread::sleep(Duration::from_secs(2));
                println!("[{}] Fine operazione", i);
                Ok(i)
            });

            match result {
                Ok(val) => println!("[{}] Risultato: {}", i, val),
                Err(e) => println!("[{}] Errore: {}", i, e),
            }

            println!("[{}] Tempo totale: {:.3?}", i, start.elapsed());
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

