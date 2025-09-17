use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let ans = (a * b).max(c * d);
    println!("{}", ans);
}
