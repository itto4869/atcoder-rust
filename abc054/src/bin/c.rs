use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    
    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let goal = (1u64 << n) - 1;
    let ans = dfs(0, 1u64 << 0, goal, &g);
    println!("{}", ans);
}

fn dfs(u: usize, used: u64, goal: u64, g: &Vec<Vec<usize>>) -> i64 {
    if used == goal {
        return 1;
    }
    let mut cnt = 0i64;
    for &v in &g[u] {
        if (used >> v) & 1 == 0 {
            cnt += dfs(v, used | (1u64 << v), goal, g);
        }
    }
    cnt
}