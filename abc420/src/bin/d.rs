use std::vec;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[i64; w]; h]
    }
    let mut dp = vec![vec![std::i64::MAX; w]; h];
    let close_grid = vec![vec!["x"; w]; h];

    dp[0][0] = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == "#"{
                dp[i][j] = -1;
            } else if grid[i][j]
        }
    }
}
