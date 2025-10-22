// No. 29 5つの合計
use std::io::{self, Write};

fn main() {
    let mut sum = 0;
    for _ in 0..5 {
        print!("input number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");
        let num: i32 = input.trim().parse().expect("conversion error");
        sum += num;
    }
    println!("sum = {}", sum);
}