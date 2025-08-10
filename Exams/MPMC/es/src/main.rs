
pub mod channel {
    use std::sync::{Arc, Condvar, Mutex};

    pub struct CircularBuffer<E: Send> {
        buf: Vec<Option<E>>,
        head: usize,
        tail: usize, 
        size: usize, 
        nelem: usize,
    }

    #[derive(PartialEq)]
    pub enum State{
        Open,
        Close,
    }

    pub struct mpmcChannel<E: Send> {
        buffer: Arc<Mutex<CircularBuffer<E>>>,
        condvar: Arc<Condvar>,
        state: State,
    }

    impl<E: Send> CircularBuffer<E> {
        pub fn new(n: usize) -> Self {
            let mut b: Vec<Option<E>> = Vec::with_capacity(n);
            for i in (0..n) {
                b.push(None);
            }

            CircularBuffer {
                buf: b,
                head: 0,
                tail: 0,
                size: n,
                nelem: 0,
            }
        }

        pub fn push(&mut self, e: E) -> Option<()> {
            if self.isfull() {
               return None;
            }
            else {
                self.buf[self.tail] = Some(e);
                self.nelem += 1;
                self.tail = (self.tail + 1) % self.size;
                return Some(());
            }
        }

        pub fn pop(&mut self) -> Option<E> {
            if self.isempty() {
                return None;
            }
            else {
                let val = self.buf[self.head].take();
                self.head = (self.head + 1) % self.size;
                self.nelem -= 1;
                return  Some(val.unwrap());
            }
        }

        pub fn isfull(&self) -> bool{
            self.nelem == self.size
        }

        pub fn isempty(&self) -> bool {
            self.nelem == 0
        }
    }

    impl<E: Send> mpmcChannel<E> {
        pub fn new(n: usize) -> Self {
            mpmcChannel {
                buffer: Arc::new(Mutex::new(CircularBuffer::new(n))),
                condvar: Arc::new(Condvar::new()),
                state: State::Open,
            }
        }

        pub fn send(&self, e: E) -> Option<()> {
            let mut buf = self.buffer.lock().unwrap();
            
            if self.state == State::Close {
                return None;
            }
            buf = self.condvar.wait_while(buf, |c| c.isfull() && self.state == State::Open).unwrap();
            buf.push(e);
            self.condvar.notify_all();
            Some(())
        }

        pub fn recv(&self) -> Option<E> {
            let mut buf = self.buffer.lock().unwrap();
            buf = self.condvar.wait_while(buf, |c| c.isempty() && self.state == State::Open).unwrap();

            if self.state == State::Open || !buf.isempty() {
                match buf.pop() {
                    Some(e) => {
                        self.condvar.notify_all();
                        return Some(e);
                    },
                    None => {
                        return None;
                    },
                }
            }
            else {
                return None;
            }
        }

        pub fn shutdown(&mut self) -> Option<()> {
            if self.state == State::Open {
                self.state = State::Close;
                self.condvar.notify_all();
                return Some(());
            }
            None
        }
    }


}

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use channel::mpmcChannel;

fn main() {
    let channel = Arc::new(mpmcChannel::new(5));

    // Clone per il produttore
    let tx = Arc::clone(&channel);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            println!("[Producer] Sending {}", i);
            if tx.send(i).is_none() {
                println!("[Producer] Channel closed");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Clone per il consumatore
    let rx = Arc::clone(&channel);
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            match rx.recv() {
                Some(val) => println!("[Consumer] Received {}", val),
                None => {
                    println!("[Consumer] Channel closed");
                    break;
                }
            }
            thread::sleep(Duration::from_millis(150));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

