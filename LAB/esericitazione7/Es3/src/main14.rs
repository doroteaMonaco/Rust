use std::sync::mpsc::{Sender, Receiver};


struct Waiter {
    my_receiver: Receiver<()>,
    others_senders: Vec<Sender<()>>,
}

struct CyclicBarrier {
    nthread: usize,
    waiters: Vec<Waiter>,
}

impl CyclicBarrier {
    fn new(n: usize) -> Self {
        let mut senders = Vec::with_capacity(n);
        let mut receivers = Vec::with_capacity(n);

        for _ in 0..n {
            let (tx, rx) = std::sync::mpsc::channel();
            senders.push(tx);
            receivers.push(rx);
        }

        let mut waiters = Vec::with_capacity(n);

        for i in 0..n {
            let mut others = Vec::with_capacity(n - 1);
            for (j, s) in senders.iter().enumerate() {
                if i != j {
                    others.push(s.clone());
                }
            }
            let w = Waiter {
                my_receiver: receivers.remove(0),
                others_senders: others,
            };
            waiters.push(w);
        }

        CyclicBarrier {
            nthread: n,
            waiters,
        }
    }

    fn get_waiter(&mut self) -> Waiter {
        self.waiters.remove(0) // Restituisce il primo Waiter disponibile e lo rimuove dalla lista spostando gli altri a sinistra
    }
}

impl Waiter {
    fn wait(&self) {
        for sender in &self.others_senders {
            let _ = sender.send(());
        }
        let _ = self.my_receiver.recv(); // Attende il segnale di un altro thread
    }
}

fn main() { 
    let mut cbarrrier = CyclicBarrier::new(3); 
 
    let mut vt = Vec::new(); 
 
    for i in 0..3 { 
        let waiter = cbarrrier.get_waiter(); 
        vt.push(std::thread::spawn(move || { 
            for j in 0..10 { 
                waiter.wait(); 
                println!("after barrier {} {}", i, j); 
            } 
        })); 
    }

    for t in vt { 
        t.join().unwrap(); 
    }
}
