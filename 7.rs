// No. 07 0 or not 0
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();

    // 入力と型変換
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let num: i32 = buf.trim().parse().expect("conversion error");

    if num == 0 {
        println!("zero");
    } else {
        println!("not zero");
    }
}