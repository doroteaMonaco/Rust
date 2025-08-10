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
