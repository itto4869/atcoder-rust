use proconio::input;

fn main() {
    input! {
        w: u64,
        a: u64,
        b: u64
    }
    if b >= a + w {
        println!("{}", b - (a + w));
    } else if b + w <= a {
        println!("{}", a - (b + w));
    } else {
        println!("0");
    }
}
