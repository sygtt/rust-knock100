// No.44 通貨換算
use std::io::{self, Write};

fn main() {
    print!("何円?: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("input error");
    let yen: i32 = input1.trim().parse().expect("conversion error");
    print!("1ドルは何円?: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("input error");
    let d_to_y: i32 = input2.trim().parse().expect("conversion error");

    let cent = (yen - yen/d_to_y * d_to_y) * 100 / d_to_y;
    println!("{}円は{}ドル{}セント", yen, yen/d_to_y, cent);
}