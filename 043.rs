// No.43 2次方程式の解の判別
use std::io::{self, Write};

fn main() {
    print!("input a: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("input error");
    let a: i32 = input1.trim().parse().expect("conversion error");
    print!("input b: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("input error");
    let b: i32 = input2.trim().parse().expect("conversion error");
    print!("input c: ");
    io::stdout().flush().unwrap();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("input error");
    let c: i32 = input3.trim().parse().expect("conversion error");

    let d = b*b - 4 * a * c;

    if d > 0 {
        println!("2つの実数解");
    }
    else if d == 0 {
        println!("重解");
    }
    else {
        println!("2つの虚数解")
    }
}