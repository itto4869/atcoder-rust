use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    if a == b || b == c || a == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
