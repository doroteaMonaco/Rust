pub mod queue {

    use std::sync::{Arc, Mutex, Condvar};
    use std::time::Instant;

   

    pub struct DelayedQueue<T: Send + Clone> {
        queue: Arc<Mutex<Vec<(T, Instant)>>>,
        condvar: Arc<Condvar>,
        size : usize,
    }

    impl<T: Send + Clone> DelayedQueue<T> {
        pub fn new() -> Self {
            DelayedQueue {
                queue: Arc::new(Mutex::new(Vec::<(T, Instant)>::new())),
                condvar: Arc::new(Condvar::new()),
                size: 0,
            }
        }

        pub fn offer(&mut self, t:T, i: Instant) {
            let mut q = self.queue.lock().unwrap();
            q.push((t, i));
            self.size += 1;
            self.condvar.notify_all();
            q.sort_by(|v1, v2| v2.1.cmp(&v1.1));
        }

        pub fn take(&mut self) -> Option<T> {
            let mut q = self.queue.lock().unwrap();

            loop {
                match q.last() {
                    Some(val) => {
                        let (t, i) = val;
                        if *i <= Instant::now() {
                            self.size -= 1;
                            let val = q.pop().unwrap();
                            return Some(val.0);
                        }
                        else {
                            let delay = *i - Instant::now();
                            let q_new = self.condvar.wait_timeout(q, delay).unwrap().0;
                            q = q_new;
                        }
                    },
                    None => {
                        return None;
                    },
                }
            }
        }

        pub fn size(&self) -> usize {
            self.size
        }
    }
}

use std::time::{Duration, Instant};
use queue::DelayedQueue;

fn main() {
    println!("Test DelayedQueue");
    
    let mut delayed_queue = DelayedQueue::<String>::new();
    
    // Aggiungi elementi con delay diversi
    let now = Instant::now();
    delayed_queue.offer("Primo".to_string(), now + Duration::from_secs(1));
    delayed_queue.offer("Secondo".to_string(), now + Duration::from_secs(2));
    delayed_queue.offer("Terzo".to_string(), now + Duration::from_secs(3));
    
    println!("Dimensione coda: {}", delayed_queue.size());
    
    // Prova a prendere elementi immediatamente (non dovrebbero essere pronti)
    for i in 1..=3 {
        println!("Tentativo {} di prendere elemento...", i);
        match delayed_queue.take() {
            Some(element) => println!("Elemento ricevuto: {}", element),
            None => println!("Nessun elemento disponibile"),
        }
    }
    
    println!("Test completato!");
}




