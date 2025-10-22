// No. 31 棒グラフ改
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");
    let num: i32 = input.trim().parse().expect("conversion error");
    let mut output = String::new();
    for i in 0..num {
        if i%5 == 4 { output.push_str("* "); }
        else { output.push('*'); }
    }
    println!("{}", output);
}