use proconio::input;
use std::cmp::{max, min};

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: i64,
        m: i64,
    };
    if (n - m).abs() > 1 {
        println!("0");
        return;
    } else if n == m {
        let mut fact = 1;
        for i in 1..=n {
            fact = fact * i % MOD;
        }
        let ans = ((fact * fact) % MOD * 2) % MOD;
        println!("{}", ans);
    } else {
        let mut fact = 1;
        for i in 1..=min(n, m) {
            fact = fact * i % MOD;
        }
        let ans = ((fact * fact) % MOD * max(n, m)) % MOD;
        println!("{}", ans);
    }
}
