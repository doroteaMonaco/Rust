
pub mod limiter {
    use std::sync::{Arc, Condvar, Mutex};

    pub struct ExecutionLimiter {
        count: Arc<Mutex<usize>>,
        condvar: Arc<Condvar>,
        nthreads: usize,
    }

    impl ExecutionLimiter {
        pub fn new(n: usize) -> Self {
            Self {
                count: Arc::new(Mutex::new(0)),
                condvar: Arc::new(Condvar::new()),
                nthreads: n,
            }
        }

        pub fn execute<F, R>(&self, f: F) -> R where F: Fn() -> Result<R, ()> + Send + Sync + 'static,
        R: Send + Sync + 'static {
            let mut cnt = self.count.lock().unwrap();

            *cnt += 1;
            if *cnt == self.nthreads {
                cnt = self.condvar.wait_while(cnt, |c| *c == self.nthreads).unwrap();
            }

            drop(cnt);
            match f() {
                Ok(r) => {
                    self.condvar.notify_all();
                    return r;
                },
                Err(_) => {
                    panic!("Error in function f");
                }
            }
        }
    }
}


use std::{
    sync::Arc,
    thread,
    time::Duration,
};
use limiter::ExecutionLimiter;

fn main() {
    const MAX_PARALLEL: usize = 3;
    const TASKS: usize = 10;

    let limiter = Arc::new(ExecutionLimiter::new(MAX_PARALLEL));
    let mut handles = Vec::new();

    for i in 0..TASKS {
        let lim = Arc::clone(&limiter);
        handles.push(thread::spawn(move || {
            // ogni task ottiene un "permesso" dal limiter
            let res: i32 = lim.execute(move || {
                println!("[Task {i}] start");
                thread::sleep(Duration::from_millis(200));
                println!("[Task {i}] end");
                Ok(i as i32 * i as i32)
            });
            println!("[Task {i}] result = {res}");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("All tasks finished.");
}

