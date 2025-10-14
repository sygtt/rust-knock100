// No. 12 ごあいさつ指定回
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    // 型推論によりi32は省略可
    let num = buf.trim().parse().expect("conversion error");

    for _ in 0..num {
        println!("Hello World!");
    }
}