use std::sync::{Arc, Mutex, Condvar};

struct BarrierState {
    count: usize,
    phase: bool, // alterna true/false per ogni ciclo
}

struct CyclicBarrier {
    nthread: usize,
    state: Arc<(Mutex<BarrierState>, Condvar)>,
}

impl CyclicBarrier {
    pub fn new(nthread: usize) -> CyclicBarrier {
        CyclicBarrier {
            nthread,
            state: Arc::new((Mutex::new(BarrierState { count: 0, phase: false }), Condvar::new())),
        }
    }

    pub fn wait(&self) {
        let (lock, cvar) = &*self.state;
        let mut state = lock.lock().unwrap();
        let local_phase = state.phase;
        state.count += 1;

        if state.count == self.nthread {
            state.count = 0;
            state.phase = !state.phase;
            cvar.notify_all();
        } else {
            while local_phase == state.phase {
                state = cvar.wait(state).unwrap();
            }
        }
    }
}


fn main() { 
    let abarrrier = Arc::new(CyclicBarrier::new(3)); 
 
    let mut vt = Vec::new(); 
 
    for i in 0..3 { 
        let cbarrier = abarrrier.clone(); 
 
        vt.push(std::thread::spawn(move || { 
            for j in 0..10 { 
                cbarrier.wait(); 
                println!("after barrier {} {}", i, j); 
            } 
        })); 
    } 
 
    for t in vt { 
        t.join().unwrap(); 
    } 
}
