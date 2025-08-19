use proconio::input;
//TODO: 未正解
fn main() {
    input! {
        n: usize,
        a: usize,
        x: [usize; n],
    }
    let sum_max = 50 * n;
    let mut dp = vec![vec![vec![0i64; sum_max + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        let xi = x[i];
        for k in 0..=i {
            for s in 0..=50 * i {
                let ways = dp[i][k][s];
                if ways == 0 { continue; }
                dp[i + 1][k][s] += ways;
                dp[i + 1][k + 1][s + xi as usize] += ways;
            }
        }
    }

    let mut ans: i64 = 0;
    for k in 1..=n {
        let target = k * a;
        if target <= sum_max {
            ans += dp[n][k][target];
        }
    }
    println!("{}", ans);
}