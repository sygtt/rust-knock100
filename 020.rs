// No. 20 割って掛ける
use std::io::{self, Write};

fn main() {
    print!("input 1st value: ");
    io::stdout().flush().unwrap();
    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("input error");
    print!("input 2nd value: ");
    io::stdout().flush().unwrap();
    let mut v2 = String::new();
    io::stdin().read_line(&mut v2).expect("input error");
    
    let v1: i32 = v1.trim().parse().expect("conversion error");
    let v2: i32 = v2.trim().parse().expect("conversion error");

    println!("result: {}", v1/v2);
    println!("result: {}", (v1/v2)*v2);
}