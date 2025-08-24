use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
    }
    let mut ans = (x + y) % 12;
    if ans == 0 {
        ans = 12;
    }
    println!("{ans}");
}
