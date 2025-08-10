

use std::sync::{Arc, Condvar, Mutex};

pub struct BarrierState {
    count: usize,
    index: usize,
    entry_phase: bool,
    exit_phase: bool,
}

pub struct RankingBarrier {
    nthread: usize,
    state: Arc<Mutex<BarrierState>>,
    cv: Condvar,
}

impl RankingBarrier {
    pub fn new(n: usize) -> Self {
        RankingBarrier {
            nthread: n,
            state: Arc::new(Mutex::new(BarrierState {
                count: 0,
                index: 1,
                entry_phase: true,
                exit_phase: false,
            })),
            cv: Condvar::new(),
        }
    }

    pub fn wait(&self) -> usize {
        let mut state = self.state.lock().unwrap();

        // Attendi che sia in fase di ingresso
        while !state.entry_phase {
            state = self.cv.wait(state).unwrap();
        }

        let rank = state.index;
        state.index += 1;
        state.count += 1;

        // Ultimo thread entra: cambia fase
        if state.count == self.nthread {
            state.entry_phase = false;
            state.exit_phase = true;
            self.cv.notify_all();
        } else {
            while state.entry_phase {
                state = self.cv.wait(state).unwrap();
            }
        }

        // Fase di uscita
        state.count -= 1;
        if state.count == 0 {
            // Ultimo a uscire: resetto la barriera
            state.index = 1;
            state.entry_phase = true;
            state.exit_phase = false;
            self.cv.notify_all();
        } else {
            while state.exit_phase {
                state = self.cv.wait(state).unwrap();
            }
        }

        rank
    }
}

use std::thread;
use std::time::Duration;

// Importa la barriera che hai definito sopra
mod barrier {
    use std::sync::{Arc, Condvar, Mutex};

    pub struct BarrierState {
        pub count: usize,
        pub index: usize,
        pub entry_phase: bool,
        pub exit_phase: bool,
    }

    pub struct RankingBarrier {
        pub nthread: usize,
        pub state: Arc<Mutex<BarrierState>>,
        pub cv: Condvar,
    }

    impl RankingBarrier {
        pub fn new(n: usize) -> Self {
            RankingBarrier {
                nthread: n,
                state: Arc::new(Mutex::new(BarrierState {
                    count: 0,
                    index: 1,
                    entry_phase: true,
                    exit_phase: false,
                })),
                cv: Condvar::new(),
            }
        }

        pub fn wait(&self) -> usize {
            let mut state = self.state.lock().unwrap();

            // Entrata: aspetta che la barriera sia in entry_phase
            while !state.entry_phase {
                state = self.cv.wait(state).unwrap();
            }

            let rank = state.index;
            state.index += 1;
            state.count += 1;

            if state.count == self.nthread {
                state.entry_phase = false;
                state.exit_phase = true;
                self.cv.notify_all();
            } else {
                while state.entry_phase {
                    state = self.cv.wait(state).unwrap();
                }
            }

            // Uscita
            state.count -= 1;

            if state.count == 0 {
                state.index = 1;
                state.entry_phase = true;
                state.exit_phase = false;
                self.cv.notify_all();
            } else {
                while state.exit_phase {
                    state = self.cv.wait(state).unwrap();
                }
            }

            rank
        }
    }
}

fn main() {
    let n_threads = 5;
    let barrier = Arc::new(barrier::RankingBarrier::new(n_threads));
    let mut handles = vec![];

    for i in 0..n_threads {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} waiting at the barrier...", i);
            thread::sleep(Duration::from_millis((10 * i) as u64)); // simula arrivo asincrono
            let rank = b.wait();
            println!("Thread {} passed the barrier with rank {}", i, rank);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have passed the barrier.");
}
