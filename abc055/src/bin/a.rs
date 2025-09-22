use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let x = n * 800;
    let y = (n / 15) * 200;
    println!("{}", x - y);
}
