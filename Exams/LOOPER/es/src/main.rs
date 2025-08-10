
pub mod looper {
    use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}};

    pub struct Looper<Msg: Send + Sync + 'static> {
        queue: Arc<Mutex<VecDeque<Option<Msg>>>>,
        condvar: Arc<Condvar>,
        jh: Option<JoinHandle<()>>,
    }

    impl<Msg: Send + Sync + 'static> Drop for Looper<Msg> {
        fn drop(&mut self) {
            let mut q = self.queue.lock().unwrap();
            q.push_back(None);
            drop(q);
            self.condvar.notify_all();
            self.jh.take().unwrap().join().unwrap();
        }
    }

    impl<Msg: Send + Sync + 'static>Looper<Msg> {
        pub fn new<F, C>(process: F, clenup: C) -> Self where F: Fn(Msg) -> () + Send + Sync + 'static, C: Fn() -> () + Send + Sync + 'static {
            let mut queue = Arc::new(Mutex::new(VecDeque::<Option<Msg>>::new()));
            let cond = Arc::new(Condvar::new());

            let cond_c = Arc::clone(&cond);
            let mut q_c = Arc::clone(&queue);

            let jh = thread::spawn(move || {
                loop {
                    let mut q = q_c.lock().unwrap();
                    if q.len() == 0 {
                        q = cond_c.wait_while(q, |q| q.len() == 0).unwrap();
                    }

                    let m = q.pop_front().unwrap();
                    drop(q);
                    match m {
                        Some(msg) => {
                            process(msg);
                        },
                        None => {
                            clenup();
                            break;
                        }
                    }
                }
            });

            Self {
                queue: queue,
                condvar: cond,
                jh: Some(jh),
            }
        } 

        pub fn send(&self, msg: Msg) {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(Some(msg));
            self.condvar.notify_all();
        }
    }
}

use std::time::Duration;

fn main() {
    let looper = looper::Looper::new(
        |msg: String| {
            println!("Processo messaggio: {}", msg);
        },
        || {
            println!("Cleanup chiamato, Looper termina.");
        },
    );

    looper.send("Messaggio 1".to_string());
    looper.send("Messaggio 2".to_string());
    looper.send("Messaggio 3".to_string());

    // Aspettiamo un po' per vedere l'elaborazione in console
    std::thread::sleep(Duration::from_secs(1));

    println!("Fine main, Looper verrà droppato e cleanup chiamato.");
}

