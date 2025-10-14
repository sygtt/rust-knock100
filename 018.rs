// No. 18 配列を入力値で初期化
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let num: i32 = buf.trim().parse().expect("conversion error");

    // 要素数10の配列をnumで初期化
    let a = [num; 10];
    
    // イテレータを用いて順番に配列の要素にアクセスする
    for i in a.iter() {
        println!("{}", i);
    }
}