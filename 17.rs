// No. 17 配列を初期化
fn main() {
    // 0で初期化された要素数10の配列を宣言
    let mut a = [0; 10];
    // for文で代入
    for i in 0..10 {
        a[i] = i;
        println!("{}", a[i]);
    }
}