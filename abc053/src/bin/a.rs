use proconio::input;

fn main() {
    input! {
        x: u64,
    }
    if x < 1200 {
        println!("ABC");
    } else {
        println!("ARC");
    }
}
