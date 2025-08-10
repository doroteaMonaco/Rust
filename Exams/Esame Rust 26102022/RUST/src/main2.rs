
pub mod cache {
    use std::{collections::HashMap, rc::Rc, sync::{Arc, Condvar, Mutex}};
    use std::hash::Hash;

    #[derive(Clone, Copy)]
    pub enum Entry<V: Copy>{
        Computing,
        Ready(V),
    }
    pub struct Cache<K: Clone + Eq + Hash + Copy, V: Copy> {
        map: Arc<Mutex<HashMap<K,Entry<V>>>>,
        condvar: Condvar,
    }

    impl<K: Clone + Eq + Hash + Copy, V: Copy> Cache<K, V> {
        pub fn new() -> Self {
            Self {
                map: Arc::new(Mutex::new(HashMap::new())),
                condvar: Condvar::new(),
            }
        }

        pub fn get<F>(&self, f: F, key: K) -> Rc<V> where F: Fn(K) -> V + Send + Sync + 'static, {
            let mut map = self.map.lock().unwrap();

            loop {
                match map.get(&key).copied() {
                    None => {
                        map.insert(key, Entry::Computing);
                        let value = f(key);
                        map.insert(key, Entry::Ready(value));
                        self.condvar.notify_all();
                        return Rc::new(value);
                    },
                    Some(Entry::Ready(val)) => {
                        return Rc::new(val);
                    },
                    Some(Entry::Computing) => {
                        map = self.condvar.wait(map).unwrap();
                    },
                };
            }
        }
    }
}

fn main() {
    let cache = cache::Cache::<u32, u32>::new();

    // Funzione di calcolo semplice
    let f = |k: u32| {
        println!("Calcolo valore per chiave {}", k);
        k * 10
    };

    // Primo accesso: calcola e inserisce
    let val1 = cache.get(f, 1);
    println!("Valore per chiave 1: {}", *val1);

    // Secondo accesso: restituisce valore già calcolato senza ricalcolare
    let val2 = cache.get(f, 1);
    println!("Valore per chiave 1 (di nuovo): {}", *val2);

    // Accesso per chiave diversa
    let val3 = cache.get(f, 2);
    println!("Valore per chiave 2: {}", *val3);
}




