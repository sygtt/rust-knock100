// No. 35 配列要素の参照
use std::io::{self, Write};

fn main() {
    print!("input index: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let index: usize = input.trim().parse().expect("conversion error");
    
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    println!("array[{}] = {}", index, array[index]);
}