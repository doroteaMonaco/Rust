
pub mod looper {
    use std::{sync::{mpsc::{channel, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};

    pub struct Looper<Message> where Message: Send + 'static {
        jh: Option<JoinHandle<()>>,
        sender: Sender<Option<Message>>,
    }

    impl<Message> Looper<Message> where Message: Send + 'static {
        pub fn new<P, C>(process:  P, clear: C) -> Self where 
        P: Fn(Message) + Sync + Send + 'static,
        C: Fn() + Sync + Send + 'static, {
            let (tx, rx) = channel::<Option<Message>>();
            let jh = thread::spawn(move || {
                loop {
                    if let Some(msg) = rx.recv().unwrap() {
                        process(msg);
                    }
                    else {
                        clear();
                        break;
                    }
                }
            });

            Self {
                jh: Some(jh),
                sender: tx,
            }
        }

        pub fn send(&self, msg: Message) {
            self.sender.send(Some(msg)).unwrap();
        }
    }

    impl<Message> Drop for Looper<Message> where Message: Send + 'static {
        fn drop(&mut self) {
            let _ = self.sender.send(None);
            self.jh.take().unwrap().join().unwrap();
        }
    }
}


use looper::Looper;
use std::{
    thread,
    time::Duration,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};

// Definizione del tipo Message
#[derive(Debug, Clone)]
struct Message {
    id: usize,
    content: String,
}

fn main() {
    // Contatore globale per tracciare i messaggi elaborati
    let counter = Arc::new(AtomicUsize::new(0));

    // Cloniamo il contatore per usarlo nella funzione process
    let counter_clone = Arc::clone(&counter);

    // Funzione di elaborazione: stampa e incrementa il contatore
    let process = move |msg: Message| {
        println!("[THREAD] Processando messaggio ID {}: {}", msg.id, msg.content);
        counter_clone.fetch_add(1, Ordering::SeqCst);
    };

    // Funzione di cleanup
    let cleanup = || {
        println!("[CLEANUP] Il looper sta per terminare.");
    };

    // Istanzia Looper
    let looper = Arc::new(Looper::new(process, cleanup));

    // Simulazione di più thread che inviano messaggi
    let mut handles = vec![];

    for i in 0..5 {
        let looper_clone = Arc::clone(&looper);
        handles.push(thread::spawn(move || {
            for j in 0..3 {
                let msg = Message {
                    id: i * 10 + j,
                    content: format!("Messaggio {} dal thread {}", j, i),
                };
                looper_clone.send(msg);
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }

    // Attende la fine dei thread
    for h in handles {
        h.join().unwrap();
    }

    // Attesa per assicurarsi che tutti i messaggi siano processati
    thread::sleep(Duration::from_millis(200));

    // Il looper verrà droppato al termine e cleanup sarà invocato
    println!(
        "[MAIN] Totale messaggi elaborati: {}",
        counter.load(Ordering::SeqCst)
    );
}



