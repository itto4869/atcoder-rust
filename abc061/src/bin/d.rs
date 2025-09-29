use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut edges = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: i64,
        }
        edges[a - 1].push((b - 1, c));
    }

    let inf = 1 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    for _ in 0..n - 1 {
        for v in 0..n {
            for &(u, c) in &edges[v] {
                if dist[v] != inf && dist[u] > dist[v] + c {
                    dist[u] = dist[v] + c;
                }
            }
        }
    }

    for _ in 0..n - 1 {
        for v in 0..n {
            for &(u, c) in &edges[v] {
                if dist[v] > dist[u] + c {
                    println!("inf");
                    return;
                }
            }
        }
    }
}
