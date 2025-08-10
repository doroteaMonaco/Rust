

pub mod barrier {
    use std::sync::{Arc, Condvar, Mutex};

    pub struct StateBarrier {
        pub count: usize,
        pub generation: usize,
        pub index: usize,
    }

    pub struct RankingBarrier {
        nthread: usize,
        state: Arc<Mutex<StateBarrier>>,
        condvar: Condvar,
    }

    impl RankingBarrier {
        pub fn new(n: usize) -> Self {
            Self {
                nthread: n,
                state: Arc::new(Mutex::new(StateBarrier {
                    count: 0,
                    generation: 0,
                    index: 1,
                })),
                condvar: Condvar::new(),
            }
        }

        pub fn wait(&self) -> usize {
            let mut state = self.state.lock().unwrap();
            let current_generation = state.generation;
            let rank = state.index;
            state.index += 1;
            state.count += 1;

            if state.count == self.nthread {
                state.count = 0;
                state.index = 1;
                state.generation += 1;
                self.condvar.notify_all();
            }

            while state.generation == current_generation {
                state = self.condvar.wait(state).unwrap();
            }

            rank
        }
    }
}

use std::sync::Arc;
use std::thread;
use barrier::RankingBarrier;

fn main() {
    let barrier = Arc::new(RankingBarrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            let rank = b.wait();
            println!("Thread {i} ha rank {rank}");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}


