

pub mod exchanger {
    use std::sync::{mpsc::Sender, mpsc::Receiver, Arc, Mutex};

    pub struct Exchanger<T: Send> {
        sender: Arc<Mutex<Sender<T>>>,
        receiver: Arc<Mutex<Receiver<T>>>,
    }

    impl<T: Send> Exchanger<T> {
        pub fn new(tx: Sender<T>, rx: Receiver<T>) -> Self {
            Exchanger { 
                sender: Arc::new(Mutex::new(tx)), 
                receiver: Arc::new(Mutex::new(rx)),
            }
        }

        pub fn exchange(&self, t: T) -> Option<T> {
            let send = self.sender.lock().unwrap();
            let rec = self.receiver.lock().unwrap();
            
            let _ = match send.send(t) {
                Ok(_) => Some(()),
                Err(_) => None,
            };

            match rec.recv() {
                Ok(m) => Some(m),
                Err(_) => None,
            }
        }
    }
}

use crate::exchanger::Exchanger;
use std::{sync::mpsc, thread, vec};

fn main() {
    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<i32>();

    let exc1 = Exchanger::new(tx1, rx2);
    let exc2 = Exchanger::new(tx2, rx1);

    let mut handle = Vec::new();

    handle.push(thread::spawn(move || {
        for i in 0..5 {
            println!("Messaggio inviato nel canale 1: {}", i);
            let msg = exc1.exchange(i).unwrap();
            println!("Messaggio ricevuto nel canale 1: {}", msg);
        }
    }));

    handle.push(thread::spawn(move || {
        for i in 0..5 {
            println!("Messaggio inviato nel canale 2: {}", i);
            let msg = exc2.exchange(i).unwrap();
            println!("Messaggio ricevuto nel canale 2: {}", msg);
        }
    }));

    for h in handle {
        h.join().unwrap();
    }
}
