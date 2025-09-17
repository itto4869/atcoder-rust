use proconio::input;
use std::collections::HashMap;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
    }
    if n == 1 {
        println!("1");
        return;
    }

    let mut exponents: HashMap<u64, u64> = HashMap::new();

    for i in 2..(n + 1) {
        let factors = prime_factorization(i as u64);
        for (base, exp) in factors {
            *exponents.entry(base).or_insert(0) += exp;
        }
    }
    let mut ans = 1u64;
    for &exp in exponents.values() {
        ans = ans * (exp + 1) % MOD;
    }
    println!("{}", ans);
}

fn prime_factorization(n: u64) -> Vec<(u64, u64)> {
    let mut n = n;
    let mut factors = vec![];
    let mut i = 2;
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count > 0 {
            factors.push((i, count));
        }
        i += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}