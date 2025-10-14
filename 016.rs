// No. 16 0でおしまい
use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    // mutableな変数とする
    let mut num: i32 = buf.trim().parse().expect("conversion error");

    while num != 0 {
        print!("input number: ");
        io::stdout().flush().unwrap();
        // バッファをクリア (これがないとエラーに)
        buf.clear();
        io::stdin().read_line(&mut buf).expect("input error");
        // 再代入 (シャドーイングだと、スコープが変わる関係でエラーに)
        num = buf.trim().parse().expect("conversion error");
    }
}