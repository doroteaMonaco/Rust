use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc, Condvar, Mutex}, thread};


struct CyclicBarrier {
    nthread: usize,
    waiter: Vec<Waiter>,
}


struct Waiter {
    my_receiver: Receiver<()>,
    others_senders: Vec<Sender<()>>,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        let mut senders = Vec::with_capacity(n);
        let mut receivers = Vec::with_capacity(n);

        // Crea tutti i canali
        for _ in 0..n {
            let (tx, rx) = channel();
            senders.push(tx);
            receivers.push(rx);
        }

        // Crea tutti i waiter
        let mut waiter = Vec::with_capacity(n);
        for i in 0..n {
            // Clona tutti i sender tranne il proprio
            let mut others = Vec::with_capacity(n - 1);
            for (j, s) in senders.iter().enumerate() {
                if i != j {
                    others.push(s.clone());
                }
            }
            waiter.push(Waiter {
                my_receiver: receivers.remove(0),
                others_senders: others,
            });
        }

        Self { nthread: n, waiter}
    }

    pub fn get_waiter(&mut self) -> Waiter {
        self.waiter.remove(0) // Restituisce il primo Waiter disponibile e lo rimuove dalla lista spostando gli altri a sinistra
    }

}


impl Waiter {
    pub fn wait(&self) {

        for sender in self.others_senders.iter() {

            let _ = sender.send(());
        }

        for _ in 0..self.others_senders.len() {
            let _ = self.my_receiver.recv();
        }
    }
}




fn main() { 
    let mut cbarrrier = CyclicBarrier::new(3); /*Non serve mettere la CyclicBarrier in un Arc perché non viene condivisa tra i thread:

Ogni thread riceve un proprio oggetto Waiter tramite get_waiter().
Dopo la creazione dei Waiter, la barriera non viene più usata dai thread.
I thread usano solo il proprio Waiter, che incapsula i canali necessari per la sincronizzazione. */
 
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

    // Aspetta che tutti i thread finiscano
    for t in vt {
        t.join().unwrap();
    }
}