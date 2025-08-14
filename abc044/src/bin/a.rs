use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        x: i64,
        y: i64,
    }
    if k > n {
        println!("{}", n * x);
    } else {
        println!("{}", k * x + (n - k) * y);
    }
}
