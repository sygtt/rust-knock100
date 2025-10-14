// No. 10 絶対値
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    // 下で書き換えられるように、mutable(変更可能)にする。
    let mut num: i32 = buf.trim().parse().expect("conversion error");

    if num < 0 {
        num = -num;
    }

    println!("absolute value is {}", num);
}