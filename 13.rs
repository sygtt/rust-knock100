// No. 13 カウントアップ
// use std::io; use std::io::Writeの2行を1行にまとめた書き方
// 詳細はThe bookの7.4項を参照
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let num = buf.trim().parse().expect("conversion error");

    // 0..=numのように書くことで、終端も含めることができる
    for i in 0..=num {
        println!("{}", i);
    }
}