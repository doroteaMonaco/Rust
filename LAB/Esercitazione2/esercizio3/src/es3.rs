use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum Result<T, E>{
    Ok(T),
    Err(E)
}

pub enum MulErr{
    Overflow {message : String},
    NegativeNumber {message : String}
}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr>{
    if(a < 0 || b < 0){
        return Result::Err(MulErr::NegativeNumber { message: String::from("Negative Number")});
    }
    if(a.checked_mul(b) == None){
        return Result::Err(MulErr::Overflow { message: String::from("Overflow")});
    }

    return Result::Ok(a as u32 * b as u32);
}

fn run_es3(){
    let a = 5;
    let b = 6;
    
    let mut result = mul(a, b);

    match result {
        Result::Ok(m) => {
            println!("{} * {} = {}", a, b, m);
        }
        Result::Err(e) =>{
            match e{
                MulErr::NegativeNumber { message } =>{
                    println!("Error: {}", message);
                }
                MulErr::Overflow { message } =>{
                    println!("Error: {}", message);
                }
            }
        }
    }

}


fn main() {
    run_es3();
}
 