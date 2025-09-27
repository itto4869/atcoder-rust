use proconio::input;

fn main() {
    input! {
        t: usize,
        m: i64,
    }
    let (fact, finv) = precompute_fact(5000);
    for _ in 0..t {
        input! {
            n: usize,
            c: [usize; n],
        }
        let mut ans = 1i64;
        let mut num = 0;
        for i in 0..n {
            num += c[i];
            ans = ans * finv[c[i]];
            println!("num: {}, ans: {}", num, ans);
        }
        ans = ans * fact[num] % m;
        println!("{}", ans);
    }
}

fn precompute_fact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = mod_mul(fact[i - 1], i as i64);
    }
    inv_fact[n] = mod_pow(fact[n], m - 2);
    for i in (1..=n).rev() {
        inv_fact[i - 1] = mod_mul(inv_fact[i], i as i64);
    }
    (fact, inv_fact)
}

fn comb(n: usize, r: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> i64 {
    if n < r {
        return 0;
    }
    mod_mul(fact[n], mod_mul(inv_fact[r], inv_fact[n - r]), m)
}

fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
    ((a as i128 * b as i128) % m as i128) as i64
}

fn mod_pow(mut x: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    while e > 0 {
        if e & 1 == 1 {
            r = mod_mul(r, x, m);
        }
        x = mod_mul(x, x, m);
        e >>= 1;
    }
    r
}