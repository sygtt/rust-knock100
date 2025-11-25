// No. 46 入場料
use std::io::{self, Write};

fn main() {
    print!("人数 ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");

    let price = if num < 5 { 600 * num }
    else if num < 20 { 550 * num }
    else { 500 * num };

    println!("料金 {}", price);
}