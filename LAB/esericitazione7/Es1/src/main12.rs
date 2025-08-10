use std::thread;
use::std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

#[derive(Clone)]
struct CountDownLatch {
    counter: Arc<(Mutex<usize>, Condvar)>, //contatore condiviso tra i thread
}


impl CountDownLatch {
    pub fn new(n: usize) -> Self {
        CountDownLatch {
            counter: Arc::new((Mutex::new(n), Condvar::new())),
        }
    }
    pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),()> {

        let (lock, cvar) = &*self.counter;
        
        let mut counter_lock = lock.lock().unwrap();

        if *counter_lock == 0 {
            Ok(())
        } else{
            match timeout {
                None => {
                    while *counter_lock > 0 {
                        counter_lock = cvar.wait(counter_lock).unwrap();
                    }
                    Ok(())
                },
                Some(timeout) => {
                    while *counter_lock > 0 {
                        let time;
                        (counter_lock, time) = cvar.wait_timeout(counter_lock, timeout).unwrap();

                        if *counter_lock == 0{
                            return Ok(());
                        }

                        if time.timed_out() {
                            return Err(());
                        }
                    }
                    Ok(())
                }
            }
        }

    }
    pub fn count_down(&self) {
        let (lock, cvar) = &*self.counter;
        let mut counter_lock = lock.lock().unwrap();

        *counter_lock -= 1;

        if *counter_lock == 0 {
            cvar.notify_all();
        }
    }
}

pub fn doSomeWork(str: &str){
    println!("{}", str);
    thread::sleep(Duration::from_millis(100));
}


pub fn demo_latch(latch: CountDownLatch) { 
    let mut handles = vec![]; 
    for _ in 0..10 { 
        let latch_clone = latch.clone();
        let h = thread::spawn(move ||{ 
            latch_clone.wait_zero(Some(Duration::from_millis(100)));
            doSomeWork("(2) lavoro che necessita driver"); 
            doSomeWork("(3) altro lavoro che non necessita driver"); 
        }); 
        handles.push(h); 
    } 
    doSomeWork("(1) prepapara il driver"); 
    latch.count_down();
    doSomeWork("(4) rilascia il driver"); 
    for h in handles { 
        let _ = h.join(); 
    } 
}

fn main() {
    let latch = CountDownLatch::new(1);
    demo_latch(latch);
}



