use proconio::{fastout, input};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    let mut ans: Vec<u64> = Vec::with_capacity(n);
    for i in (0..n).rev().step_by(2) {
        ans.push(a[i]);
    }

    let start = if a.len() % 2 == 0 {
        0
    } else {
        1
    };
    for i in (start..n).step_by(2) {
        ans.push(a[i]);
    }

    println!("{}", ans.iter().format(" "));
}
