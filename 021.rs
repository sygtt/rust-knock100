// No. 21 5より大きく20より小さい
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("input error");
    let num: i32 = num.trim().parse().expect("conversion error");
    
    if num > 5 && num < 20 {
        println!("OK");
    }
}