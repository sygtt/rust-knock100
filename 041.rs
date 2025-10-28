// No.41 1桁の自然数?
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");

    if num > 0 && num <= 9 {
        println!("{} is a single figure.", num);
    } else {
        println!("{} is not a single figure.", num);
    }
}