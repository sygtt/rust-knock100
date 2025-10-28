// No. 39 もっと・配列要素の参照
fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    for i in 0..9 {
        println!("{}", array[i]-array[i+1]);
    }
}