// No. 15 2ずつカウントアップ
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let num = buf.trim().parse().expect("conversion error");

    for i in (0..=num).step_by(2) {
        println!("{}", i);
    }
}