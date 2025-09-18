use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    
    let mut counter = HashMap::new();
    for _ in 0..n {
        input! {
            a: u64,
        }
        *counter.entry(a).or_insert(0) += 1;
    }

    let mut even = 0;
    let mut odd = 0;
    for (_, v) in counter {
        if v % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    let mut ans = 0;
    ans += (even / 2) * 2;
    if even % 2 == 1 && odd > 0 {
        ans += 1;
        odd -= 1;
    }
    ans += odd;
    println!("{}", ans);
}