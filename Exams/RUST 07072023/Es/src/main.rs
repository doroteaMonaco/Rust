

pub mod queue {
    use std::{collections::{btree_set::Intersection, VecDeque}, sync::{Arc, Condvar, Mutex}, time::Instant};

    //Metto Option<T> perchè quando scade inserisco None
    pub struct DelayedQueue<T: Send + Copy> {
        queue: Arc<Mutex<Vec<(T, Instant)>>>,
        condvar: Arc<Condvar>,
    }

    impl<T: Send + Copy> DelayedQueue<T> {
        pub fn new() -> Self {
            Self {
                queue: Arc::new(Mutex::new(Vec::new())),
                condvar: Arc::new(Condvar::new()),
            }
        }

        pub fn offer(&mut self, t: T, i: Instant){
            let mut q = self.queue.lock().unwrap();
            q.push((t, i));
            self.condvar.notify_all();
            q.sort_by( |a, b| b.1.cmp(&a.1)); //decrescente perchè la pop toglie l'ultimo valore
        }

        pub fn take(&self) -> Option<T> {
            let mut q = self.queue.lock().unwrap();

            loop {
                if q.is_empty() {
                    return None;
                }
                
                let val = q.last().unwrap();
                let (t, i) = val;
                if *i <= Instant::now() {
                    return Some(*t);
                }
                else {
                    let delay = *i - Instant::now();
                    let (new_q, _) = self.condvar.wait_timeout(q, delay).unwrap();
                    q = new_q;
                }
            }
        }

        pub fn size(&self) -> usize {
            let q = self.queue.lock().unwrap();
            q.len()
        }
    }
}


use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use queue::DelayedQueue;

fn main() {
    let queue = Arc::new(DelayedQueue::new());

    // thread che inserisce elementi
    let producer = {
        let q = queue.clone();
        thread::spawn(move || {
            q.offer("prima", Instant::now() + Duration::from_secs(2));
            q.offer("seconda", Instant::now() + Duration::from_secs(1));
        })
    };

    // thread che consuma elementi
    let consumer = {
        let q = queue.clone();
        thread::spawn(move || {
            while let Some(msg) = q.take() {
                println!("Ricevuto: {}", msg);
            }
        })
    };

    producer.join().unwrap();
    // aspetta un po' che il consumer prenda i messaggi
    thread::sleep(Duration::from_secs(3));

    // Poiché consumer può bloccarsi su take() se la coda è vuota,
    // eventualmente si può interrompere il consumer in altro modo (non implementato qui).
}

