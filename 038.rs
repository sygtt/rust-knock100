// No. 38 さらに・配列要素の参照
fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    let mut val: usize = 0;
    for _ in 0..10 {
        println!("{}", array[val]);
        val = array[val];
    }
}