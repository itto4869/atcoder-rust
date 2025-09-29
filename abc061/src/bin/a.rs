use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }
    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
