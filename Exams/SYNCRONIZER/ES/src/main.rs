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

mod synchronizer_corretto;

use std::{sync::Arc, thread, time::Duration};
use synchronizer_corretto::Synchronizer;

fn main() {
    println!("=== Test Synchronizer Corretto ===");

    let sync = Arc::new(Synchronizer::new(|d1, d2| {
        println!("Process: d1 = {}, d2 = {}", d1, d2);
    }));

    let sync1 = Arc::clone(&sync);
    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Invio dato porta 1: {}", i * 10);
            sync1.data_from_first_port(i as f32 * 10.0);
            thread::sleep(Duration::from_millis(200));
        }
        println!("Thread 1 terminato");
    });

    let sync2 = Arc::clone(&sync);
    let handle2 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Invio dato porta 2: {}", i * 100);
            sync2.data_from_second_port(i as f32 * 100.0);
            thread::sleep(Duration::from_millis(250));
        }
        println!("Thread 2 terminato");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Tutti i thread sono terminati correttamente!");
}
