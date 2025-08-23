use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
    }
    let ans = k * (k - 1).pow(n - 1);
    println!("{ans}");
}
