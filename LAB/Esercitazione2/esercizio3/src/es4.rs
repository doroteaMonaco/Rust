use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

struct Node{
    name: String, 
    size: u32, 
    count: u32,
}

impl Node{
    pub fn new(name: &str) -> Node{
        Node{
            name: name.to_string(),
            size: 0, 
            count: 0 
        }
    }

    //self consuma la struct e quindi devo ritornare una nuova struct
    pub fn size(self, n: u32) -> Self{
        Self{ 
            name: self.name, 
            size: n, 
            count: self.count 
        }
    }

    pub fn count(self, c: u32) -> Self{
        Self{
            name: self.name, 
            size: self.size,
            count: c
        }
    }

    pub fn to_string(&self) -> String{
        return String::from(format!("{} {} {}", self.name, self.size, self.count));
    }

    pub fn grow(&mut self) -> () {
        self.size += 1;
    }

    pub fn inc(&mut self) -> (){
        self.count += 1;
    }

}

fn run_es4(){
    let node = Node::new(&String::from("node")).size(10).count(5);
    println!("{}", node.to_string());
    println!("{}", node.size(20).to_string());
    let node2 = Node::new(&String::from("node2")).size(10).count(5);
    println!("{}", node2.to_string());
    println!("{}", node2.count(10).to_string());
    let mut node3 = Node::new(&String::from("node3")).size(10).count(5);
    node3.grow();
    println!("{}", node3.to_string());
    node3.inc();
    println!("{}", node3.to_string());
}


fn main() {
    run_es4();
}
 