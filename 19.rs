// No. 19 配列に入力値を格納
use std::io::{self, Write};

fn main() {
    // 可変で要素数5の配列(0で初期化)
    let mut a = [0; 5];

    // a.len()で、配列aの要素数を取得
    for i in 0..a.len() {
        print!("input number: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("input error");
        a[i] = buf.trim().parse().expect("conversion error");
    }

    for i in a.iter() {
        println!("{}", i);
    }
}