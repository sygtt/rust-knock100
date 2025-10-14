// No. 05 四則演算
use std::io;
use std::io::Write;

fn main() {
    // 1つ目の数を入力
    print!("input 1st number: ");
    io::stdout().flush().unwrap();

    let mut buf1 = String::new();
    io::stdin().read_line(&mut buf1).expect("input error");
    
    // 2つ目の数を入力
    print!("input 2nd number: ");
    io::stdout().flush().unwrap();

    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).expect("input error");

    // String型をi32型に変換
    let num1: i32 = buf1.trim().parse().expect("conversion error");
    let num2: i32 = buf2.trim().parse().expect("conversion error");
    
    println!("和: {}", num1 + num2);
    println!("差: {}", num1 - num2);
    println!("積: {}", num1 * num2);
    println!("商: {}, 余り: {}", num1 / num2, num1 % num2);
}