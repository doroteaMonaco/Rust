
pub mod countdown {
    use std::{sync::{Arc, Condvar, Mutex}, time::Duration};


    pub struct CounDownLock {
        count: Arc<Mutex<usize>>,
        condvar: Arc<Condvar>,
    }

    impl CounDownLock {
        pub fn new(n: usize) -> Self {
            Self {
                count: Arc::new(Mutex::new(n)),
                condvar: Arc::new(Condvar::new()),
            }
        }

        pub fn count_down(&self) {
            let mut cnt = self.count.lock().unwrap();

            if *cnt > 0 {
                *cnt -= 1;
            }
            if *cnt == 0 {
                self.condvar.notify_all();
            }
        }

        pub fn wait(&self) {
            let mut cnt = self.count.lock().unwrap();
            cnt = self.condvar.wait_while(cnt, |c| *c != 0).unwrap();
        }

        pub fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult {
            let mut cnt = self.count.lock().unwrap();
            self.condvar.wait_timeout_while(cnt, d, |c| *c != 0).unwrap().1
        }
    }
}

use std::thread;
use std::time::Duration;
use countdown::CounDownLock;

fn main() {
    // Aspettiamo che 3 thread completino il lavoro
    let countdown = CounDownLock::new(3);
    let shared = std::sync::Arc::new(countdown);

    for i in 0..3 {
        let worker_latch = std::sync::Arc::clone(&shared);
        thread::spawn(move || {
            println!("[Thread {}] Lavoro in corso...", i);
            thread::sleep(Duration::from_millis(500 + i * 200));
            println!("[Thread {}] Lavoro completato, count_down()", i);
            worker_latch.count_down();
        });
    }

    println!("[Main] In attesa che tutti i thread completino...");
    shared.wait(); // Bloccante finché il contatore non arriva a zero
    println!("[Main] Tutti i thread hanno completato. Proseguo!");
}

