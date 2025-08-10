use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;

pub struct MyChannel<T> {
    queue: Arc<(Mutex<VecDeque<Item<T>>>, Condvar)>,
    size: usize,
    closed: Mutex<bool>,
}

pub enum Item<T> {
    Value(T),
    Stop,
}

impl<T> MyChannel<T> {
    pub fn new(size: usize) -> Self {
        MyChannel {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
            size,
            closed: Mutex::new(false),
        }
    }

    pub fn write(&self, item: T) -> Result<(), ()> {
        let (lock, cvar) = &*self.queue;
        let mut items = lock.lock().unwrap();

        if *self.closed.lock().unwrap() {
            return Err(());
        }

        while items.len() == self.size {
            items = cvar.wait(items).unwrap();
        }

        items.push_back(Item::Value(item));
        cvar.notify_all();
        Ok(())
    }

    pub fn read(&self) -> Result<T, ()> {
        let (lock, cvar) = &*self.queue;
        let mut items = lock.lock().unwrap();

        loop {
            while items.is_empty() {
                if *self.closed.lock().unwrap() {
                    // Se il canale è chiuso e il buffer è vuoto, esci
                    return Err(());
                }
                items = cvar.wait(items).unwrap();
            }

            // Qui il buffer NON è vuoto: restituisci il valore
            match items.pop_front().unwrap() {
                Item::Value(item) => {
                    cvar.notify_all();
                    return Ok(item);
                }
                Item::Stop => {
                    return Err(());
                }
            }
        }
    }

    pub fn stop(&self) -> Result<(), &'static str> {
        let (lock, cvar) = &*self.queue;
        let mut items = lock.lock().unwrap();

        if *self.closed.lock().unwrap() {
            return Err("Channel is closed");
        }

        items.push_back(Item::Stop);
        cvar.notify_all();
        Ok(())
    }
    
    pub fn close(&self) {
        let (lock, cvar) = &*self.queue;
        let mut items = lock.lock().unwrap();

        if *self.closed.lock().unwrap() {
            return;
        }

        *self.closed.lock().unwrap() = true;

        while !items.is_empty() {
            items.pop_front();
        }

        cvar.notify_all();
    }
}

fn main() {
    let channel = Arc::new(MyChannel::new(5));

    let producer_channel = Arc::clone(&channel);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            producer_channel.write(i).unwrap();
            println!("Produced: {}", i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
        // Invia il segnale di stop invece di close
        producer_channel.stop().unwrap();
    });

    let consumer_channel = Arc::clone(&channel);
    let consumer = thread::spawn(move || {
        loop {
            match consumer_channel.read() {
                Ok(value) => println!("Consumed: {}", value),
                Err(_) => break, // Termina quando riceve Item::Stop o il canale è chiuso
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
