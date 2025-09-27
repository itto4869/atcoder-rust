use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n]
    }
    let mut diff = 0;
    let mut dp = vec![0; n];
    dp[0] = a[0];
    for i in 1..n {
        dp[i] = dp[i - 1] + a[i];
    }
    for _ in 0..q {
        input! {
            m: usize,
            t: [usize; m]
        }
        if t.len() == 1 {
            diff += t[0];
            diff %= n;
        } else {
            let mut start = t[0] + diff;
            let mut end = t[1] + diff;
            let mut ans = 0;
            if start > n {
                start -= n;
            }
            if end > n {
                end -= n;
            }
            if start < end {
                ans += dp[end - 1];
                if start > 1 {
                    ans -= dp[start - 2];
                }
            } else if start > end{
                ans += dp[n - 1];
                if start > 1 {
                    ans -= dp[start - 2];
                }
                ans += dp[end - 1];
            } else {
                ans += dp[end - 1];
                if start > 1 {
                    ans -= dp[start - 2];
                }
            }
            println!("{}", ans);
        }
    }
}
