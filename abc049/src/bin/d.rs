use std::vec;
use std::collections::HashMap;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize, l: usize,
    }
    let mut uf_road = UnionFind::new(n);
    let mut uf_rail = UnionFind::new(n);

    for _ in 0..k {
        input! {
            p: usize,
            q: usize,
        }
        uf_road.union(p - 1, q - 1);
    }

    for _ in 0..l {
        input! {
            r: usize,
            s: usize,
        }
        uf_rail.union(r - 1, s - 1);
    }

    let mut pair = vec![(0usize, 0usize); n];
    for i in 0..n {
        let rr = uf_road.find(i);
        let tr = uf_rail.find(i);
        pair[i] = (rr, tr);
    }
    let mut cnt: HashMap<(usize, usize), usize> = HashMap::new();
    for &p in &pair {
        *cnt.entry(p).or_insert(0) += 1;
    }

    let mut ans = Vec::with_capacity(n);
    for &p in &pair {
        ans.push(cnt[&p]);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
