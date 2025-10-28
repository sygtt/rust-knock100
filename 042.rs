// No.42 小さい順?
use std::io::{self, Write};

fn main() {
    print!("input number1: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("input error");
    let num1: i32 = input1.trim().parse().expect("conversion error");
    print!("input number2: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("input error");
    let num2: i32 = input2.trim().parse().expect("conversion error");
    print!("input number3: ");
    io::stdout().flush().unwrap();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("input error");
    let num3: i32 = input3.trim().parse().expect("conversion error");

    if num1 <= num2 && num2 <= num3 {
        println!("OK");
    } else {
        println!("NG");
    }
}