use proconio::input;

fn main() {
    input! {
        x: u64,
    }
    let mut sum = 0;
    let mut n = 1;
    while sum < x {
        sum += n;
        n += 1;
    }
    println!("{}", n - 1);
}
