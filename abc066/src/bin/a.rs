use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut value: [u64; 3]
    }
    value.sort();
    println!("{}", value[0] + value[1]);
}
