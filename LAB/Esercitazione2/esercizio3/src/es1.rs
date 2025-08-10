use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn write_file1 (filename: &str) -> std::io::Result<()> {
    let string = match fs::read_to_string(filename){
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(err);
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("text.txt")
        .expect("Unable to open file");

    for _ in 0..9 {
        writeln!(file, "{}", string).expect("Unable to write to the file");
    }

    Ok(())
}

fn run_es3_1(){
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];

    write_file1(filename).expect("Error while writing file");
}

fn write_file2 (filename1 : &str) -> std::io::Result<()> {
    let data : Vec<u8> = match fs::read(filename1){
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(err);
        }
    };

    for d in &data{
        print!("{}", *d as char);
    }
    println!("\n");

    for da in &data{
        print!("{:02x} ", *da);
    }

    Ok(())
}

fn run_es3_2(){
    let filename1 : String = String::from("text1.txt");

    write_file2(&filename1).expect("Error while writing file");
}


fn main() {
    run_es3_1();
    run_es3_2();
}
