// No. 47 値の入れ替え
use std::io::{self, Write};

fn main() {
    print!("input a: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a: i32 = input.trim().parse().expect("conversion error1");
    print!("input b: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut b: i32 = input.trim().parse().expect("conversion error2");
    
    let buf = a;
    a = b;
    b = buf;
    
    println!("a = {}, b = {}", a, b);
}