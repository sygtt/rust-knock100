// No. 45 タクシー料金
use std::io::{self, Write};

fn main() {
    print!("距離 ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let mut distance: i32 = input.trim().parse().expect("conversion error");

    let mut price = 610;
    
    while distance > 1700 {
        price += 80;
        distance -= 313;
    }
    
    println!("金額 {}", price);
}