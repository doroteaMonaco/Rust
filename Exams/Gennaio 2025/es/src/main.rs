
pub mod executor {
    use std::{sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::{self, Duration, Instant}};

    #[derive(PartialEq, Copy, Clone)]
    pub enum State{
        Open,
        Close,
    }
    pub struct DelayedExecutor <F: FnOnce () -> () + Send + 'static + Clone> {
        tasks: Arc<Mutex<(Vec<(F, Instant)>, State)>>,
        condvar: Arc<Condvar>,
        worker: Option<JoinHandle<()>>,
    }

    impl<F: FnOnce() -> () + Send + 'static + Clone> Drop for DelayedExecutor<F> {
        fn drop(&mut self) {
            self.close(true);
            self.worker.take().unwrap().join().unwrap();
        }
    }

    impl<F: FnOnce() -> () + Send + 'static + Clone> DelayedExecutor<F> {
        pub fn new() -> Self {

            let mut tasks = Arc::new(Mutex::new((Vec::<(F, Instant)>::new(), State::Open)));
            let condvar = Arc::new(Condvar::new());
            let cond_c = Arc::clone(&condvar);
            let mut task_c = tasks.clone();
            let jh = thread::spawn(move ||{
                loop {
                    let mut t = task_c.lock().unwrap();
                    if t.1 == State::Open {
                        t = cond_c.wait_while(t, |ta| ta.0.len() != 0).unwrap();
                        let now = Instant::now();
                        let (f, i) = t.0.last().unwrap();
                        if *i <= now {
                            let val = (*t).0.pop().unwrap();
                            val.0();
                        }
                        else {
                            let d = i.duration_since(Instant::now());
                            t = cond_c.wait_timeout(t, d).unwrap().0;
                        }
                    }
                    else {
                        break;
                    }
                }
            });

            Self {
                tasks: tasks,
                condvar: condvar,
                worker: Some(jh),
            }
        }

        pub fn execute(&self, f: F, delay: Duration) -> bool {
            let mut t = self.tasks.lock().unwrap();
            if t.1 == State::Close {
                return false;
            }
            let i = Instant::now() + delay;
            t.0.push((f, i));
            t.0.sort_by(|a,b|  b.1.cmp(&a.1));
            self.condvar.notify_all();
            return true;
        }

        pub fn close(&self, drop_pending_tasks: bool) {
            let mut t = self.tasks.lock().unwrap();
            t.1 = State::Close;
            if drop_pending_tasks == true {
                t.0.clear();
            }
            self.condvar.notify_all();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
