// No. 32 5の倍数でbar
fn main() {
    for i in 1..=20 {
        if i%5 == 0 {
            println!("bar");
        } else {
            println!("{}", i);
        }
    }
}