
pub mod dispatcher {
    use std::sync::{self, mpsc::{self, Receiver, Sender}};

    pub struct Dispatcher<Msg: Clone> {
        senders: Vec<Sender<Msg>>,
    }

    pub struct Subscriber<Msg: Clone> {
       receiver: Receiver<Msg>,
    }

    impl<Msg: Clone> Dispatcher<Msg> {
        pub fn new() -> Self {
            Self {
                senders: Vec::<Sender<Msg>>::new(),
            }
        }

        pub fn dispatch(&self, msg: Msg){
            for s in (0..self.senders.len()) {
                self.senders[s].send(msg.clone()).unwrap();
            }
        }

        pub fn subscribe(&mut self) -> Subscriber<Msg> {
            let (tx, rx) = mpsc::channel::<Msg>();
            self.senders.push(tx);
            Subscriber { receiver: rx }
        }
    }

    impl<Msg: Clone> Subscriber<Msg> {
        pub fn read(&self) -> Option<Msg> {
            match self.receiver.recv() {
                Ok(msg) => {
                    return  Some(msg);
                }, 
                Err(_) => {
                    return None;
                }
            }
        }
    }
}


use std::thread;
use std::time::Duration;

use dispatcher::{Dispatcher, Subscriber};

fn main() {
    let mut dispatcher = Dispatcher::new();

    // Due sottoscrittori
    let sub1 = dispatcher.subscribe();
    let sub2 = dispatcher.subscribe();

    // Dispatch di messaggi in un thread separato
    let d = std::sync::Arc::new(dispatcher);
    let d_clone = std::sync::Arc::clone(&d);

    thread::spawn(move || {
        for i in 0..5 {
            d_clone.dispatch(format!("Messaggio {}", i));
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Primo subscriber
    let h1 = thread::spawn(move || {
        for _ in 0..5 {
            if let Some(msg) = sub1.read() {
                println!("[SUB1] Ricevuto: {}", msg);
            }
        }
    });

    // Secondo subscriber
    let h2 = thread::spawn(move || {
        for _ in 0..5 {
            if let Some(msg) = sub2.read() {
                println!("[SUB2] Ricevuto: {}", msg);
            }
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

