use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans = (n * (n + 1)) / 2;
    print!("{}", ans);
}
