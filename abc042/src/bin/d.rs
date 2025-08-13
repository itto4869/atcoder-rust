use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    }

    let max_n = h + w + 5;
    let (fact, inv_fact) = precompute_fact(max_n);

    let mut ans = 0i64;
    for i in 0..(h - a) {
        let left = comb(b - 1 + i, i, &fact, &inv_fact);
        let right = comb((w - b - 1) + (h - 1 - i), h - 1 - i, &fact, &inv_fact);
        ans += mod_mul(left, right);
        if ans >= MOD {
            ans -= MOD;
        }
    }

    print!("{}", ans % MOD);
}

fn mod_mul(a: i64, b: i64) -> i64 {
    ((a as i128 * b as i128) % MOD as i128) as i64
}

fn mod_pow(mut x: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    while e > 0 {
        if e & 1 == 1 {
            r = mod_mul(r, x);
        }
        x = mod_mul(x, x);
        e >>= 1;
    }
    r
}

fn precompute_fact(n: usize) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = mod_mul(fact[i - 1], i as i64);
    }
    inv_fact[n] = mod_pow(fact[n], MOD - 2);
    for i in (1..=n).rev() {
        inv_fact[i - 1] = mod_mul(inv_fact[i], i as i64);
    }
    (fact, inv_fact)
}

fn comb(n: usize, r: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> i64 {
    if n < r {
        return 0;
    }
    mod_mul(fact[n], mod_mul(inv_fact[r], inv_fact[n - r]))
}