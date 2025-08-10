use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

enum Error{
    Simple{
        time: SystemTime
    },
    Complex{
        time: SystemTime,
        message: String,
    }
}

fn print_error (e: Error) -> (){
    match e{
        Error::Simple {time} => {
            let duration = time.duration_since(UNIX_EPOCH).expect("Error time");
            println!("Simple error at {}", duration.as_secs());
        }
        Error::Complex {time, message} => {
            let duration = time.duration_since(UNIX_EPOCH).expect("Error time");
            println!("Complex error at {} with message {}", duration.as_secs(), message);
        }
    }
}

fn run_es2(){
    let e  = Error::Complex{time : SystemTime::now(), message : String::from("Error complex")};
    print_error(e);
}

fn main() {
    run_es2();
}