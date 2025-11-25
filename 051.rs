// No. 51 お支払い
use std::io::{self, Write};

fn main() {
    print!("input money: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let mut money: i32 = input.trim().parse().expect("conversion error");

    let hnum: i32 = money / 100;
    money -= hnum * 100;
    
    let tnum: i32 = money / 10;
    money -= tnum * 10;

    println!("100円玉{}枚, 10円玉{}枚, 1円玉{}枚", hnum, tnum, money);
}