
pub mod channel {
    use std::sync::{mpsc::{self, Receiver, SendError, Sender}, Arc, Condvar, Mutex};

    pub struct MultiChannel {
        senders: Arc<Mutex<Vec<Sender<u8>>>>,
    }

    impl MultiChannel {
        pub fn new() -> Self {
            Self {
                senders: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn subscribe(&self) -> Receiver<u8> {
            let (tx, rx) = mpsc::channel::<u8>();
            let mut send = self.senders.lock().unwrap();
            send.push(tx);
            rx
        }

        pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
            let mut send = self.senders.lock().unwrap();

            if send.len() == 0 {
                return Err(SendError(data));
            }

            send.retain(|s| s.send(data).is_ok());
            Ok(())
                        
        }
    }
}


use std::thread;
use std::time::Duration;
use crate::channel::MultiChannel; // Sostituisci `your_crate_name` con il nome del tuo crate/module

fn main() {
    let channel = MultiChannel::new();

    // Creiamo 3 ricevitori che ascoltano in thread separati
    for i in 0..3 {
        let rx = channel.subscribe();
        thread::spawn(move || {
            for val in rx {
                println!("[Receiver {}] Ricevuto: {}", i, val);
            }
        });
    }

    // Piccola pausa per assicurarsi che tutti i thread siano partiti
    thread::sleep(Duration::from_millis(100));

    // Inviamo 5 messaggi
    for i in 0..5 {
        match channel.send(i) {
            Ok(_) => println!("[Main] Inviato: {}", i),
            Err(e) => println!("[Main] Errore nell'invio: {}", e.0),
        }
        thread::sleep(Duration::from_millis(200));
    }

    // Attesa per permettere ai thread di stampare prima che il main termini
    thread::sleep(Duration::from_secs(1));
}

