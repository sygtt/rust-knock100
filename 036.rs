// No. 36 続・配列要素の参照
use std::io::{self, Write};

fn main() {
    print!("input index1: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("input error");
    let index1: usize = input1.trim().parse().expect("conversion error");
    print!("input index2: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("input error");
    let index2: usize = input2.trim().parse().expect("conversion error");
    
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    
    println!("{} * {} = {}", array[index1], array[index2], array[index1]*array[index2]);
}