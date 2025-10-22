// No. 27 1からnまでの和
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");
    if num <= 0 {
        println!("0");
    } else {
        let mut sum = 0;
        for i in 0..=num {
            sum += i;
        }
        println!("{}", sum);
    }
}