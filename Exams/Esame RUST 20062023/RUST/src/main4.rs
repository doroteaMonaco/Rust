
pub mod mpmcChannel {
    use std::{fmt::format, sync::{Arc, Condvar, Mutex}};

    //i metodi per il CircularBuffer sono: new, isClosed, isEmpty, push, pop e shutdown per settarlo a chiuso.
    pub struct CircularBuffer<E: Send + Clone> {
        vec: Vec<Option<E>>,
        head: usize,
        tail: usize,
        closed: bool,
        size: usize,
        capacity: usize,
    }

    impl<E: Send + Clone> CircularBuffer<E> {
        pub fn new(n: usize) -> Self {
            Self {
                vec: vec![None; n],
                head: 0,
                tail: 0,
                closed: false,
                size: 0,
                capacity: n,
            }
        }

        pub fn is_closed(&self) -> bool {
            self.closed == true
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        pub fn is_full(&self) -> bool {
            self.size == self.capacity
        }

        //head solo per fare pop, tail per push
        //non devo controllare i valori di head e tail ma solo se è presente un Some
        pub fn push(&mut self, e: E) -> Result<(),String> {
            if self.is_full() {
                return Err(format!("Buffer pieno, impossibile inserire!"));
            }
            self.vec[self.tail] = Some(e);
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;
            Ok(())
        }

        pub fn pop(&mut self) -> Option<E> {
            if self.is_empty() {
                return None;
            }

            let e = self.vec[self.head].take()?;
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;
            Some(e)
        }

        pub fn close(&mut self) {
            self.closed = true;
        }
    }

    #[derive(Clone)]
    pub struct MpMcChannel<E: Send + Clone> {
        data: Arc<(Mutex<CircularBuffer<E>>, Condvar)>,
    }

    impl <E: Send + Clone> MpMcChannel<E> {
        pub fn new(n: usize) -> Self {
            let circular_buffer: CircularBuffer<E>  = CircularBuffer::new(n);
            Self {
                data: Arc::new((Mutex::new(circular_buffer), Condvar::new())),
            }
        }

        pub fn send(&self, e: E) -> Option<()> {
            let (lock, cv) = &*self.data;
            let mut d = lock.lock().unwrap();

            if d.is_full() {
                d = cv.wait(d).unwrap();
            }
            else if d.is_closed(){
                return None;
            }
            else {
                d.push(e);
                cv.notify_all();
            }
            Some(())
        }

        pub fn recv(&mut self) -> Option<E> {
            let (lock, cv) = &*self.data;
            let mut d = lock.lock().unwrap();

            if d.is_empty() {
                d = cv.wait(d).unwrap();
            }
            else if d.is_closed(){
                return None;
            }
            
            let e = match d.pop() {
                Some(e) => e,
                None => return None,
            };

            cv.notify_all();
            Some(e)
        }

        pub fn shutdown(&self) -> Option<()> {
            let (lock, cv) = &*self.data;
            let mut d = lock.lock().unwrap();
            
            d.close();
            cv.notify_all();

            Some(())
        }
    }
}

use std::thread;
use std::time::Duration;

// Importa il modulo
use crate::mpmcChannel::MpMcChannel;

fn main() {
    let mut channel = MpMcChannel::new(3);

    // Clona il canale per il produttore
    let sender = channel.clone();

    // Produttore
    let producer = thread::spawn(move || {
        for i in 0..5 {
            sender.send(i).unwrap();
            println!("Inviato: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        sender.shutdown();
        println!("Produttore terminato.");
    });

    // Consumatore
    let consumer = thread::spawn(move || {
        loop {
            match channel.recv() {
                Some(val) => println!("Ricevuto: {}", val),
                None => {
                    println!("Canale chiuso, consumatore termina.");
                    break;
                }
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

