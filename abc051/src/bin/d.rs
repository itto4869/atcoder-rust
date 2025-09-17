use proconio::input;

const INF: i64 = 1 << 29;
fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(usize, usize, i64); m], 
    }
    let mut dist = vec![vec![INF; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    for (a, b, c) in &edge {
        dist[a - 1][b - 1] = *c;
        dist[b - 1][a - 1] = *c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = 0;
    for (a, b, c) in edge {
        if c > dist[a - 1][b - 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
