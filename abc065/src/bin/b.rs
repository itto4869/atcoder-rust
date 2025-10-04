use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            a: usize,
        }
        graph[i].push(a - 1);
    }

    let mut seen = vec![false; n];
    dfs(&graph, 0, &mut seen);
    if seen[1] {
        let mut count = 1;
        let mut next = graph[0][0];
        loop {
            if next == 1 {
                println!("{}", count);
                return;
            } else {
                next = graph[next][0];
                count += 1;
            }
        }
    } else {
        println!("-1");
        return;
    }
}

fn dfs(graph: &Vec<Vec<usize>>, s: usize, seen: &mut Vec<bool>) {
    seen[s] = true;

    for v in &graph[s] {
        if seen[*v] { continue };
        dfs(graph, *v, seen);
    }
}