use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(u64, u64); n],
    }
    let mut dp = vec![0u64; (w + 1) as usize];
    for i in 0..n {
        let (weight, value) = wv[i];
        for j in (weight..=w).rev() {
            dp[j as usize] = dp[j as usize].max(dp[(j - weight) as usize] + value as u64);
        }
    }
    let ans = dp[w as usize];
    println!("{}", ans);
}

