// No. 03 入力
// 参考サイト: https://terminart-express.site/archives/4141
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    // 入力の前にメッセージを改行なしで表示させたいのでflushする
    io::stdout().flush().unwrap();

    // 入力をbufで受け取る
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");

    // bufについてる改行コードを除去して出力
    println!("your number is {}", buf.trim_end());
}