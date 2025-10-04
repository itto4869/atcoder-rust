use proconio::input;

#[derive(Clone)]
struct DSU {
    p: Vec<usize>,
    s: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self { Self { p: (0..n).collect(), s: vec![1; n] } }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] == x { x } else { self.p[x] = self.find(self.p[x]); self.p[x] }
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b { return false; }
        if self.s[a] < self.s[b] { std::mem::swap(&mut a, &mut b); }
        self.p[b] = a; self.s[a] += self.s[b]; true
    }
}

fn main() {
    input! { n: usize, xy: [(i64, i64); n] }

    let mut xs: Vec<(i64, i64, usize)> = xy.iter().enumerate()
        .map(|(i, &(x,y))| (x, y, i)).collect();
    let mut ys = xs.clone();
    xs.sort_by_key(|&(x,_,_)| x);
    ys.sort_by_key(|&(_,y,_)| y);

    let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(2*(n-1));
    for i in 0..n-1 {
        let (x1,y1,a) = xs[i];     let (x2,y2,b) = xs[i+1];
        edges.push(((x2-x1).abs().min((y2-y1).abs()), a, b));
        let (x1,y1,a) = ys[i];     let (x2,y2,b) = ys[i+1];
        edges.push(((x2-x1).abs().min((y2-y1).abs()), a, b));
    }
    edges.sort_by_key(|e| e.0);

    let mut dsu = DSU::new(n);
    let mut ans: i64 = 0;
    for (w,u,v) in edges {
        if dsu.unite(u,v) { ans += w; }
    }
    println!("{}", ans);
}
