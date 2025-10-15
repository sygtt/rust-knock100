// No. 22 -10以下または10以上
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("input error");
    let num: i32 = num.trim().parse().expect("conversion error");
    
    if num < -10 || num >= 10 {
        println!("OK");
    }
}