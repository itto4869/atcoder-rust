use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }
    if (b - a) == (c - b) {
        println!("YES");
    } else {
        println!("NO");
    }
}
