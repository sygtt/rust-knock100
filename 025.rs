// No. 25 -10未満?、-10以上0未満?、0以上?
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("input error");
    let num: i32 = num.trim().parse().expect("conversion error");
    
    if num < -10 {
        println!("range 1");
    }
    else if num >= -10 && num < 0 {
        println!("range 2");
    }
    else {
        println!("range 3");
    }
}