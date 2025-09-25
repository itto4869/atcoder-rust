use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [i64; n],
        y: [i64; m]
    }
    let mut xsum = 0;
    let mut ysum = 0;
    for i in 0..n-1 {
        let cnt = (i as i64 + 1) * (n as i64 - 1 - i as i64) % MOD;
        xsum = xsum + (x[i + 1] - x[i]) * cnt % MOD;
    }
    for i in 0..m-1 {
        let cnt = (i as i64 + 1) * (m as i64 - 1 - i as i64) % MOD;
        ysum = ysum + (y[i + 1] - y[i]) * cnt % MOD;
    }
    let ans = (xsum % MOD) * (ysum % MOD) % MOD;
    println!("{}", (ans + MOD) % MOD);
}
