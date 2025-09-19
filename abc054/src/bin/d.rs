use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        ma: usize,
        mb: usize,
        items: [(usize, usize, i32); n], 
    }

    let max_a = items.iter().map(|&(a, _, _)| a).sum::<usize>();
    let max_b = items.iter().map(|&(_, b, _)| b).sum::<usize>();

    const INF: i32 = 1_000_000_000;
    let mut dp = vec![vec![INF; max_b + 1]; max_a + 1];
    dp[0][0] = 0;

    for &(a, b, c) in &items {
        for x in (a..=max_a).rev() {
            for y in (b..=max_b).rev() {
                if dp[x - a][y - b] != INF {
                    dp[x][y] = min(dp[x][y], dp[x - a][y - b] + c);
                }
            }
        }
    }

    let mut ans = INF;
    let mut k = 1usize;
    loop {
        let x = ma * k;
        let y = mb * k;
        if x > max_a || y > max_b { break; }
        ans = min(ans, dp[x][y]);
        k += 1;
    }

    println!("{}", if ans == INF { -1 } else { ans });

}
