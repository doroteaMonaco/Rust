/*Esercizio: BatchedExecutor
Implementa una struct BatchedExecutor in Rust che consente di accodare funzioni da eseguire in batch, ogni N secondi, su un thread separato.

Specifica funzionale
La struct si chiama BatchedExecutor ed è thread-safe.

Offre i seguenti metodi:

impl BatchedExecutor {
    fn new(batch_interval: Duration) -> Self;
    
    fn submit<F: FnOnce() + Send + 'static>(&self, f: F) -> bool;
    
    fn close(&self, drop_pending: bool);
}

Comportamento
new(batch_interval):

Crea un nuovo BatchedExecutor che esegue batch ogni batch_interval.

submit(f):

Se l’esecutore è ancora aperto, accoda la funzione f da eseguire nel prossimo batch e restituisce true.

Se l’esecutore è chiuso, restituisce false.

close(drop_pending):

Ferma l’esecutore.

Se drop_pending == true, le funzioni non ancora eseguite vengono scartate.

Se drop_pending == false, le funzioni accodate vengono comunque eseguite nel prossimo batch prima di terminare.

Dopo close(), ogni futura submit() restituisce false.

Vincoli
Le funzioni vanno accumulate in un buffer e poi eseguite tutte insieme ogni batch_interval.

L’esecuzione dei batch avviene su un thread secondario, che va terminato pulitamente (→ JoinHandle).

La chiusura (close) deve essere sicura e senza race condition.

Alla distruzione (Drop) della struct, il thread deve essere chiuso come in close(true).

 */

pub mod executor {
    use std::{sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::{Duration}};


    #[derive(PartialEq)]
    pub enum State{
        Open,
        Close,
    }

    pub struct BatchedExecutor {
        tasks: Arc<Mutex<(Vec<Box <dyn FnOnce() + Send + 'static>>, State)>>,
        condvar: Arc<Condvar>,
        jh: Option<JoinHandle<()>>,
    }

    impl Drop for BatchedExecutor {
        fn drop(&mut self) {
            self.close(true);
            self.jh.take().unwrap().join().unwrap();
        }
    }

    impl BatchedExecutor {
        fn new(batch_interval: Duration) -> Self {
            let mut tasks = Arc::new(Mutex::<(Vec<Box<dyn FnOnce() + Send + 'static>>, State)>::new((Vec::new(), State::Open)));
            let condvar = Arc::new(Condvar::new());

            let mut task_c = Arc::clone(&tasks);
            let cond_c = Arc::clone(&condvar);

            let jh = thread::spawn(move || {
                loop {
                    let mut t = task_c.lock().unwrap();
            
                    if t.1 == State::Close {
                        break;
                    }
                    else {
                        while t.0.len() == 0 {
                            t = cond_c.wait(t).unwrap();
                        }
                        t = cond_c.wait_timeout(t, batch_interval).unwrap().0;

                        let f = t.0.pop().unwrap();
                        f();
                    }
                }
            });

            Self {
                tasks: tasks,
                condvar: condvar,
                jh: Some(jh),
            }
        }
    
        fn submit<F: FnOnce() + Send + 'static>(&self, f: F) -> bool {
            let mut t = self.tasks.lock().unwrap();
            if t.1 == State::Close {
                return false;
            }
            t.0.push(Box::new(f));
            self.condvar.notify_all();
            return true;
        }
        
        fn close(&self, drop_pending: bool) {
            let mut t = self.tasks.lock().unwrap();
            t.1 = State::Close;
            if drop_pending == true {
                t.0.clear();
                self.condvar.notify_all();
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}
