
pub mod dispatcher {
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::sync::{Arc, Mutex};

    pub struct Dispatcher<Msg: Clone>{
        senders: Arc<Mutex<Vec<Sender<Msg>>>>,
    }

    pub struct Subscription<Msg: Clone>{
        receiver: Receiver<Msg>,
    }

    impl<Msg: Clone> Dispatcher<Msg> {
        pub fn new() -> Self {
            Self {
                senders: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn dispatch(&self, msg: Msg){
            let mut sends = self.senders.lock().unwrap();

            for i in (0..sends.len()){
                match sends[i].send(msg.clone()) {
                    Ok(_) => {()},
                    Err(_) => {sends.remove(i);},
                }
            }

        }

        pub fn subscribe(&self) -> Subscription<Msg> {
            let (tx, rx) = mpsc::channel();
            let mut send = self.senders.lock().unwrap();
            send.push(tx);
            Subscription { receiver: rx, }
        }

    }

    impl<Msg:Clone> Subscription<Msg>{
        pub fn read(&self) -> Option<Msg>{
            match self.receiver.recv() {
                Ok(msg) => Some(msg),
                Err(_) => None,
            }
        }

    }
}

use std::thread;
use crate::dispatcher::{Dispatcher, Subscription};
use std::time::Duration;

fn main() {
    // 1) Creiamo il dispatcher
    let dispatcher = Dispatcher::new();

    // 2) Due subscription
    let subscriber1 = dispatcher.subscribe();
    let subscriber2 = dispatcher.subscribe();

    // 3) Subscriber 1
    let handle1 = thread::spawn(move || {
        while let Some(msg) = subscriber1.read() {
            println!("🔵 Subscriber 1 ha ricevuto: {}", msg);
        }
        println!("🔵 Subscriber 1: canale chiuso, esco.");
    });

    // 4) Subscriber 2
    let handle2 = thread::spawn(move || {
        while let Some(msg) = subscriber2.read() {
            println!("🟢 Subscriber 2 ha ricevuto: {}", msg);
        }
        println!("🟢 Subscriber 2: canale chiuso, esco.");
    });

    // 5) Publisher
    let pub_handle = thread::spawn(move || {
        for i in 0..5 {
            println!("✉️ Dispatching: {}", i);
            dispatcher.dispatch(i);
            thread::sleep(Duration::from_millis(100));
        }
        // quando questa closure termina, `dispatcher` viene droppato
        println!("🏁 Publisher: terminato e dispatcher droppato.");
    });

    // 6) Aspettiamo tutti
    pub_handle.join().unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
}

