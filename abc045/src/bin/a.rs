use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        h: u64,
    }
    println!("{}", (a + b) * h / 2);
}
