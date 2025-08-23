use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        abc: [i64; 3],
    }
    let ans = abc.into_iter().collect::<HashSet<_>>().len();
    println!("{ans}");
}
