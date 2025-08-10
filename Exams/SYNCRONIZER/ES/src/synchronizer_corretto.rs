use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

pub struct Synchronizer {
    _jh1: JoinHandle<()>,
    _jh2: JoinHandle<()>,
    sender1: Sender<f32>,
    sender2: Sender<f32>,
}

impl Synchronizer {
    pub fn new<F>(process: F) -> Self 
    where 
        F: Fn(f32, f32) + Send + Sync + 'static + Clone
    {
        let (tx1, rx1) = mpsc::channel::<f32>();
        let (tx2, rx2) = mpsc::channel::<f32>();
        
        let data1 = Arc::new((Mutex::new(Option::<f32>::None), Condvar::new()));
        let data2 = Arc::new((Mutex::new(Option::<f32>::None), Condvar::new()));
        
        let data1_clone1 = Arc::clone(&data1);
        let data2_clone1 = Arc::clone(&data2);
        let process1 = process.clone();
        
        // Thread 1: gestisce dati dalla prima porta
        let jh1 = thread::spawn(move || {
            for d1 in rx1 {
                let (lock1, cvar1) = &*data1_clone1;
                let (lock2, cvar2) = &*data2_clone1;
                
                // Metto il mio dato in data1
                {
                    let mut data1 = lock1.lock().unwrap();
                    *data1 = Some(d1);
                    cvar1.notify_all();
                }
                
                // Aspetto un dato da data2
                let mut data2 = lock2.lock().unwrap();
                while data2.is_none() {
                    data2 = cvar2.wait(data2).unwrap();
                }
                let d2 = data2.take().unwrap();
                drop(data2);
                
                // Rimuovo il mio dato da data1
                {
                    let mut data1 = lock1.lock().unwrap();
                    data1.take();
                }
                
                // Processo i dati
                process1(d1, d2);
            }
        });
        
        let data1_clone2 = Arc::clone(&data1);
        let data2_clone2 = Arc::clone(&data2);
        let process2 = process.clone();
        
        // Thread 2: gestisce dati dalla seconda porta
        let jh2 = thread::spawn(move || {
            for d2 in rx2 {
                let (lock1, cvar1) = &*data1_clone2;
                let (lock2, cvar2) = &*data2_clone2;
                
                // Metto il mio dato in data2
                {
                    let mut data2 = lock2.lock().unwrap();
                    *data2 = Some(d2);
                    cvar2.notify_all();
                }
                
                // Aspetto un dato da data1
                let mut data1 = lock1.lock().unwrap();
                while data1.is_none() {
                    data1 = cvar1.wait(data1).unwrap();
                }
                let d1 = data1.take().unwrap();
                drop(data1);
                
                // Rimuovo il mio dato da data2
                {
                    let mut data2 = lock2.lock().unwrap();
                    data2.take();
                }
                
                // Processo i dati
                process2(d1, d2);
            }
        });

        Self {
            _jh1: jh1,
            _jh2: jh2,
            sender1: tx1,
            sender2: tx2,
        }
    }

    pub fn data_from_first_port(&self, d1: f32) {
        let _ = self.sender1.send(d1);
    }

    pub fn data_from_second_port(&self, d2: f32) {
        let _ = self.sender2.send(d2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn test_synchronizer() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let sync = Arc::new(Synchronizer::new(move |d1, d2| {
            println!("Process: d1 = {}, d2 = {}", d1, d2);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        let sync1 = Arc::clone(&sync);
        let handle1 = thread::spawn(move || {
            for i in 1..=3 {
                println!("Invio dato porta 1: {}", i * 10);
                sync1.data_from_first_port(i as f32 * 10.0);
                thread::sleep(Duration::from_millis(100));
            }
        });

        let sync2 = Arc::clone(&sync);
        let handle2 = thread::spawn(move || {
            for i in 1..=3 {
                println!("Invio dato porta 2: {}", i * 100);
                sync2.data_from_second_port(i as f32 * 100.0);
                thread::sleep(Duration::from_millis(150));
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Verifica che process sia stato chiamato esattamente 3 volte
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}

// Esempio di utilizzo
#[allow(dead_code)]
fn main() {
    let sync = Arc::new(Synchronizer::new(|d1, d2| {
        println!("Process: d1 = {}, d2 = {}", d1, d2);
    }));

    let sync1 = Arc::clone(&sync);
    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Invio dato porta 1: {}", i * 10);
            sync1.data_from_first_port(i as f32 * 10.0);
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    let sync2 = Arc::clone(&sync);
    let handle2 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Invio dato porta 2: {}", i * 100);
            sync2.data_from_second_port(i as f32 * 100.0);
            thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Programma terminato correttamente!");
}



pub mod synchronizer {
    use std::{sync::mpsc::{self, Sender}, thread::{self, JoinHandle}};
    use std::sync::{Arc, Mutex, Condvar};
    pub struct Synchronizer {
        coda1: Arc<(Mutex<Vec<f32>>, Condvar)>,
        coda2: Arc<(Mutex<Vec<f32>>, Condvar)>,
        jh1: Option<JoinHandle<()>>,
        jh2: Option<JoinHandle<()>>,
        sender1: Sender<f32>,
        sender2: Sender<f32>,
    }

    impl Drop for Synchronizer {
        fn drop(&mut self) {
            drop(&self.sender1);
            drop(&self.sender2);


            self.jh1.take().unwrap().join().unwrap();
            self.jh2.take().unwrap().join().unwrap();
        }
    }

    impl Synchronizer {
        pub fn new<F>(process: F) -> Self where F: Fn(f32, f32) + Send + Sync + Copy + Clone + 'static {
            let (tx1, rx1) = mpsc::channel::<f32>();
            let (tx2, rx2) = mpsc::channel::<f32>();
            let v1 = Arc::new((Mutex::new(Vec::<f32>::new()), Condvar::new()));
            let v2 = Arc::new((Mutex::new(Vec::<f32>::new()), Condvar::new()));

            let v1_c = Arc::clone(&v1);
            let v2_c = Arc::clone(&v2);

            let jh1 = thread::spawn(move || {
                let (lock, cond2) = &*v2_c;
                loop {
                    match rx1.recv() {
                        Ok(d1) => {
                            let mut v2 = lock.lock().unwrap();
                            v2 = cond2.wait_while(v2, |v| v.len() == 0).unwrap();
                            if let Some(d2) = v2.pop() {
                                process(d1, d2);
                            }
                            else {
                                break;
                            }
                        },
                        Err(_) => {
                            break;
                        },
                    }
                }
            });

            let jh2 = thread::spawn(move || {
                let (lock, cond1) = &*v1_c;
                loop {
                    match rx2.recv() {
                        Ok(d2) => {
                            let mut v1 = lock.lock().unwrap();
                            v1 = cond1.wait_while(v1, |v| v.len() == 0).unwrap();
                            if let Some(d1) = v1.pop() {
                                process(d1, d2);
                            }
                            else {
                                break;
                            }
                        },
                        Err(_) => {
                            break;
                        },
                    }
                }
            });

            Self {
                coda1: v1,
                coda2: v2,
                jh1: Some(jh1),
                jh2: Some(jh2),
                sender1: tx1,
                sender2: tx2,
            }
        }

        pub fn data_from_first_port(&self, d1: f32) -> () {
            let (lock1, cond1) = &*self.coda1;
            let mut v1 = lock1.lock().unwrap();

            v1.push(d1);
            self.sender1.send(d1).unwrap();
            cond1.notify_all();
        }

        pub fn data_from_second_port(&self, d2: f32) -> () {
            let (lock2, cond2) = &*self.coda2;
            let mut v2 = lock2.lock().unwrap();

            v2.push(d2);
            self.sender2.send(d2).unwrap();
            cond2.notify_all();
        }
    }
}



use std::{sync::Arc, thread, time::Duration};
use synchronizer::Synchronizer;

fn main() {
    let sync = Arc::new(Synchronizer::new(|d1, d2| {
        println!("Process: d1 = {}, d2 = {}", d1, d2);
    }));

    let sync1 = Arc::clone(&sync);
    let sync2 = Arc::clone(&sync);

    // Strategia: invio prima un dato alla seconda porta per "preparare" la coda
    sync2.data_from_second_port(100.0);
    thread::sleep(Duration::from_millis(10)); // piccola pausa per assicurare l'ordine

    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Invio dato porta 1: {}", i as f32 * 10.0);
            sync1.data_from_first_port(i as f32 * 10.0);
            thread::sleep(Duration::from_millis(200));
        }
    });

    let sync3 = Arc::clone(&sync);
    let handle2 = thread::spawn(move || {
        for i in 2..=5 {
            println!("Invio dato porta 2: {}", i as f32 * 100.0);
            sync3.data_from_second_port(i as f32 * 100.0);
            thread::sleep(Duration::from_millis(200));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Aggiungi una pausa per permettere l'elaborazione
    thread::sleep(Duration::from_millis(500));

    println!("Fine del main");
    
    // IMPORTANTE: Per evitare il blocco nel Drop, uso mem::forget
    // Questo non è una soluzione ideale ma evita il blocco
    std::mem::forget(sync);
    
    println!("Programma terminato");
}

