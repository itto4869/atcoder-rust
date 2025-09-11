use proconio::input;
use std::collections::HashMap;
const MOD: i64 = 1_000_000_007;

fn f(n: u128, memo: &mut HashMap<u128, i64>) -> i64 {
    if n == 0 { return 1; }
    if let Some(&v) = memo.get(&n) { return v; }

    let a = f(n / 2, memo);
    let b = f((n - 1) / 2, memo);
    let c = if n >= 2 { f((n - 2) / 2, memo) } else { 0 };

    let mut res = a;
    res += b; if res >= MOD { res -= MOD; }
    res += c; if res >= MOD { res -= MOD; }
    memo.insert(n, res);
    res
}

fn main() {
    input! {
        n: u128,
    }
    let mut memo = HashMap::new();
    println!("{}", f(n, &mut memo));
}
