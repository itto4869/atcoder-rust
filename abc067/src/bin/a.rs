use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64
    }
    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
