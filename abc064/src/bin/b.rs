use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    };
    let mut max_a = 0;
    let mut min_a = std::u64::MAX;
    for ai in a {
        max_a = max(max_a, ai);
        min_a = min(min_a, ai);
    }
    println!("{}", max_a - min_a);
}
