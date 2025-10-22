// No. 30 棒グラフ
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");
    let mut output = String::new();
    for _ in 0..num {
        output.push('*');
    }
    println!("{}", output);
}