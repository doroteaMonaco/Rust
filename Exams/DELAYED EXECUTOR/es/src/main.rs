

pub mod executor {
    use std::{sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant}};

    #[derive(PartialEq)]
    pub enum State {
        Open, 
        Close,
    }
    pub struct DelayedExecutor {
        queue: Arc<Mutex<Vec<(Box<dyn Fn() + Send + Sync + 'static>,Instant)>>>,
        condvar: Arc<Condvar>,
        state: Arc<Mutex<State>>,
        thread: Option<JoinHandle<()>>,
    }

    impl Drop for DelayedExecutor {
        fn drop(&mut self) {
            self.close(true);
            self.thread.take().unwrap().join().unwrap();
        }
    }

    impl DelayedExecutor {
        pub fn new() -> Self {
            let mut queue = Arc::new(Mutex::new(Vec::<(Box<dyn Fn() + Send + Sync + 'static>, Instant)>::new()));
            let cond = Arc::new(Condvar::new());
            let state = Arc::new(Mutex::new(State::Open));

            let mut q = Arc::clone(&queue);
            let cond_c = Arc::clone(&cond);
            let state_c = Arc::clone(&state);

            let jh = thread::spawn(move || {
                loop {
                    let mut q_c = q.lock().unwrap();
                    let mut s = state_c.lock().unwrap();

                    if *s == State::Open {
                        q_c = cond_c.wait_while(q_c, |q| q.len() == 0).unwrap();
                    }
                    else {
                        break;
                    }

                    match q_c.last() {
                        Some(tup) => {
                            if tup.1 <= Instant::now() {
                                (tup.0)();
                                q_c.pop();
                            }
                            else {
                                let d = tup.1 - Instant::now();
                                q_c = cond_c.wait_timeout(q_c, d).unwrap().0;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
            });

            Self {
                queue: queue,
                condvar: cond,
                state: state,
                thread: Some(jh),
            }
            
        }

        pub fn execute<F>(&self, f: F, delay: Duration) -> bool where F: Fn() + Send + Sync + 'static {
            let mut q = self.queue.lock().unwrap();
            let state = self.state.lock().unwrap();

            if *state == State::Open {
                drop(state);
                let inst = Instant::now() + delay;
                q.push((Box::new(f), inst));
                q.sort_by(|a, b| b.1.cmp(&a.1));
                drop(q);
                self.condvar.notify_all();
                return true;
            }
            else {
                drop(state);
                return false;
            }
        }

        pub fn close(&self, drop_pending_tasks: bool) {
            let mut q = self.queue.lock().unwrap();
            let mut state = self.state.lock().unwrap();
            if drop_pending_tasks == true {
                *state = State::Close;
                self.condvar.notify_all();
                q.clear();
            }
        }
    }
}




use executor::DelayedExecutor;   // percorso completo
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    // Istante iniziale per misurare i ritardi
    let start = Instant::now();

    // Crea il DelayedExecutor
    let exec = DelayedExecutor::new();

    // Task 1: dopo 1 secondo
    exec.execute(
        {
            let start = start.clone();
            move || {
                println!(
                    "[{:>4} ms] Task 1 eseguito!",
                    Instant::now().duration_since(start).as_millis()
                );
            }
        },
        Duration::from_secs(1),
    );

    // Task 2: dopo 1.5 secondi
    exec.execute(
        {
            let start = start.clone();
            move || {
                println!(
                    "[{:>4} ms] Task 2 eseguito!",
                    Instant::now().duration_since(start).as_millis()
                );
            }
        },
        Duration::from_millis(1500),
    );

    // Task 3: dopo 2 secondi
    exec.execute(
        {
            let start = start.clone();
            move || {
                println!(
                    "[{:>4} ms] Task 3 eseguito!",
                    Instant::now().duration_since(start).as_millis()
                );
            }
        },
        Duration::from_secs(2),
    );

    // Mantieni vivo il main abbastanza a lungo
    thread::sleep(Duration::from_secs(3));

    // Quando `exec` esce dallo scope, drop() chiude e unisce il thread
}
