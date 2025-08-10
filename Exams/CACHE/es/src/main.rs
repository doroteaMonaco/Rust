
pub mod cache {
    use std::{collections::HashMap, sync::{Arc, Condvar, Mutex}};
    use std::hash::Hash;

    #[derive(PartialEq)]
    pub enum State<V>{
        Busy,
        Available(Arc<V>)
    }
    pub struct Cache <K: Clone + Copy + Eq + Hash, V: PartialEq + Clone + Copy> {
        map: Arc<Mutex<HashMap<K,State<V>>>>,
        condvar: Arc<Condvar>,
    }

    impl<K: Clone  + Copy + Eq + Hash, V: PartialEq + Clone + Copy> Cache<K, V> {
        pub fn new() -> Self {
            Cache {
                map: Arc::new(Mutex::new(HashMap::new())),
                condvar: Arc::new(Condvar::new()),
            }
        }

        pub fn get<F>(&self, key: K, f: F) -> Arc<V> where F: Fn(K) -> V + Send + Sync + 'static {
            let mut m = self.map.lock().unwrap();

            if let Some(State::Available(val)) = m.get(&key) {
                return Arc::clone(&val);
            }
            
            if let Some(State::Busy) = m.get(&key) {
                m = self.condvar.wait(m).unwrap();
                return  self.get(key, f);
            }
            
            m.insert(key, State::Busy);
            let v = f(key.clone());
            m.insert(key.clone(), State::Available(Arc::new(v)));
            self.condvar.notify_all();
            return Arc::new(v);
        }
    }
}


use std::sync::Arc;
use std::thread;
use std::time::Duration;

 // Assicurati che il tuo modulo sia nello stesso progetto

use cache::Cache;

fn main() {
    let cache = Arc::new(Cache::<u32, u64>::new());

    // Funzione costosa (simulate con sleep + stampa)
    let compute = |key: u32| {
        println!("[Compute] Calcolo valore per chiave {}", key);
        thread::sleep(Duration::from_secs(2)); // Simula operazione costosa
        (key as u64) * 10
    };

    let mut handles = vec![];

    for i in 0..3 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            println!("[Thread {}] Richiede chiave 42", i);
            let val = cache_clone.get(42, compute);
            println!("[Thread {}] Ha ottenuto: {}", i, *val);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Tutti i thread hanno terminato.");
}


/*
impl<K: Clone + Copy + Eq + Hash, V: Clone + Copy> Cache<K, V> {
    pub fn get<F>(&self, key: K, f: F) -> Arc<V>
    where
        F: Fn(K) -> V + Send + Sync + 'static,
    {
        // 1. prendi il lock sulla mappa
        let mut map = self.map.lock().unwrap();

        // 2. attende finché la chiave è Busy (spurious‑wake safe)
        map = self.condvar
            .wait_while(map, |m| matches!(m.get(&key), Some(State::Busy)))
            .unwrap();

        // 3. se ora è Available, restituisci subito il valore
        if let Some(State::Available(val)) = map.get(&key) {
            return Arc::clone(val);
        }

        // 4. se non esiste ancora => questo thread calcola il valore
        map.insert(key, State::Busy);          // riserva la chiave
        drop(map);                             // 🔓 libera il lock

        let val = Arc::new(f(key));            // calcolo fuori dal lock

        // 5. rientra, pubblica il risultato e sveglia chi attende
        let mut map = self.map.lock().unwrap();
        map.insert(key, State::Available(Arc::clone(&val)));
        self.condvar.notify_all();
        val
    }
}

 */