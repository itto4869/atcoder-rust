use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        x: [u64; n],
    }
    let mut ans = 0;
    for i in 0..(n - 1) {
        let d = x[i + 1] - x[i];
        ans += b.min(a * d);
    }
    println!("{}", ans);
}
