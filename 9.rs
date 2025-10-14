// No. 09 正? 負? 0?
use std::io;
use std::io::Write;

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("input error");
    let num: i32 = buf.trim().parse().expect("conversion error");

    if num > 0 {
        println!("positive");
    }
    else if num < 0 {
        println!("negative");
    }
    else {
        println!("zero");
    }
}