// No.40 even or odd
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");

    if num%2 == 0 {
        println!("{} is even.", num);
    } else {
        println!("{} is odd.", num);
    }
}