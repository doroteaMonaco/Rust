
pub mod processor {
    use std::{sync::{mpsc::{self, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};

    #[derive(PartialEq)]
    pub enum State {
        Open,
        Close,
    }
    pub struct Processor<T: Copy + Send + Sync + 'static> {
        consumer: Option<JoinHandle<()>>,
        tx: Option<Sender<T>>,
        close: Arc<Mutex<State>>,
    }

    impl<T: Copy + Send + Sync + 'static> Processor<T> {
        pub fn new<F>(f: F) -> Self where F: Fn(T) -> () + Sync + Send + 'static
        {
            let (tx, rx) = mpsc::channel::<T>();
            let jh = thread::spawn(move || {
                loop {
                    match rx.recv() {
                        Ok(msg) => {
                            f(msg);
                        },
                        Err(_) => break,
                    }
                }
            });

            Self {
                consumer: Some(jh),
                tx: Some(tx),
                close: Arc::new(Mutex::new(State::Open)),
            }
        }

        pub fn send(&self, t: T) {
            let close = self.close.lock().unwrap();
            if *close == State::Open {
                if let Some(s) = &self.tx {
                    s.send(t).unwrap();
                }
            }
            else {
                panic!("Canale chiuso");
            }
        }

        pub fn close(&mut self) {
            let mut close = self.close.lock().unwrap();
            *close = State::Close;
            self.tx.take();
            self.consumer.take().unwrap().join().unwrap();
        }
    }
}



use std::{thread, time::Duration};
use processor::Processor;
         // il modulo che hai già definito

fn main() {
    // 1. Creo il Processor e definisco il lavoro del consumer
    let mut processor = Processor::new(|val: u32| {
        println!("[Consumer] Elaboro {}", val);
        thread::sleep(Duration::from_millis(300));
    });

    // 2. Avvio più thread produttori in parallelo
    thread::scope(|scope| {
        for producer_id in 0..3 {
            // catturo un riferimento al Processor
            let p = &processor;
            scope.spawn(move || {
                // Ogni producer invia 5 messaggi
                for i in 1..=5 {
                    let value = producer_id * 100 + i;      // valori distinti per ogni producer
                    println!("[Producer {producer_id}] Invia {value}");
                    p.send(value);
                    thread::sleep(Duration::from_millis(100));
                }
            });
        }
        // Tutti i thread all'interno di `scope` devono finire qui prima di uscire dal blocco
    });

    // 3. Quando tutti i produttori hanno finito, chiudiamo il Processor
    println!("[Main] Chiudo il processore...");
    processor.close();
    println!("[Main] Finito.");
}


