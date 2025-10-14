// No. 04 入力と計算
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");

    // String型をi32型に変換
    let num: i32 = buf.trim().parse().expect("conversion error");
    
    println!("answer = {}", num * 3);
}