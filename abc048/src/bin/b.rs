use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    let start = if a % x == 0 { a / x } else { a / x + 1 };
    let end = b / x;
    println!("{}", end - start + 1);
}
