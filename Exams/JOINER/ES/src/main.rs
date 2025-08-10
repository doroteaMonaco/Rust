
pub mod joiner {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, Condvar};
    use rand::thread_rng;
    use rand::Rng;

    pub struct Sensor();

    impl Sensor {
        pub fn generate() -> i32 {
            let mut rng = thread_rng();
            rng.gen_range(0..100)
        }
    } 

    pub struct State {
        generation: usize,
        count: usize,
        map: HashMap<usize, HashMap<i32, f32>>,
    }

    pub struct Joiner {
        state: Arc<Mutex<State>>,
        condvar: Arc<Condvar>,
        nthreads: usize,
    }

    impl Joiner  {
        pub fn new(n: usize) -> Self {
            Self {
                state: Arc::new(Mutex::new(State {
                    generation: 0,
                    count: 0,
                    map: HashMap::<usize, HashMap<i32, f32>>::new(),
                })),
                condvar: Arc::new(Condvar::new()),
                nthreads: n,
            }
        }

        pub fn supply(&self, key: i32, value: f32) -> HashMap<i32, f32> {
            let mut state = self.state.lock().unwrap();
            let generation = state.generation;
            
            

            let entry = state.map.entry(generation).or_insert_with(HashMap::new);
            entry.insert(key, value);   
            state.count += 1;

            if state.count == self.nthreads {
                state.count = 0;
                state.generation += 1;
                self.condvar.notify_all();
            }
            else {
                state = self.condvar.wait_while(state, |s| s.generation == generation).unwrap();
            }

            let completed = generation;
            let map_return = state.map.get(&completed).unwrap().clone();
            map_return
        }
    }
}


use std::{sync::Arc, thread, time::Duration};
use joiner::{Joiner, Sensor};

fn main() {
    const N: usize = 4;       // Numero di thread concorrenti
    const ROUNDS: usize = 3;  // Numero di cicli di raccolta

    // Creiamo un Joiner condiviso tra i thread
    let joiner = Arc::new(Joiner::new(N));

    let mut handles = Vec::with_capacity(N);

    for thread_id in 0..N {
        let joiner_clone = Arc::clone(&joiner);

        // Spawn di un thread worker
        let handle = thread::spawn(move || {
            for round in 0..ROUNDS {
                // Genera un valore simulato dal sensore
                let value = Sensor::generate() as f32;
                println!("[Thread {} - Round {}] Invio valore: {}", thread_id, round, value);

                // Invio il valore e ricevo la mappa aggregata per la tornata corrente
                let map = joiner_clone.supply(thread_id as i32, value);

                // Stampo la mappa ricevuta
                println!(
                    "[Thread {} - Round {}] Mappa ricevuta ({} elementi): {:?}",
                    thread_id,
                    round,
                    map.len(),
                    map
                );

                // Simula un ritardo tra un round e l'altro
                thread::sleep(Duration::from_millis(100));
            }
        });

        handles.push(handle);
    }

    // Aspetta che tutti i thread terminino
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n[Main] Tutti i thread hanno completato le tornate.");
}
