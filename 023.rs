// No. 23 -5以上10未満
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("input error");
    let num: i32 = num.trim().parse().expect("conversion error");
    
    if num >= -5 && num < 10 {
        println!("OK");
    } else {
        println!("NG");
    }
}