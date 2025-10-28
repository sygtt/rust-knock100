// No. 33 入力値抜き
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");

    for i in 1..=9 {
        if i != num {
            println!("{}", i);
        }
    }
}