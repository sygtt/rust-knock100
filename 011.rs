// No. 11 ごあいさつ10回
fn main() {
    // Rustコンパイラは未使用変数に対して警告を出す。変数を意図的に使わない場合には_と書く。
    for _ in 0..10 {
        println!("Hello World!");
    }
}