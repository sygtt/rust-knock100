// No. 28 nの階乗
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num = input.trim().parse().expect("conversion error");
    let mut fact = 1;
    if num > 0 {
        for i in 1..=num {
            fact *= i;
        }
    }
    println!("factrial = {}", fact);
}