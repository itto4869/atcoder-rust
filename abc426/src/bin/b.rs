use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    };
    let s: Vec<char> = s.chars().collect();
    let mut counter = HashMap::new();
    for c in s {
        *counter.entry(c).or_insert(0) += 1;
    }
    for (k, v) in counter {
        if v == 1 {
            println!("{}", k);
        }
    }
}
