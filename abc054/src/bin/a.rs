use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    if a == b {
        println!("Draw");
    } else if a != 1 && b != 1 && a > b {
        println!("Alice");
    } else if a != 1 && b != 1 && a < b {
        println!("Bob");
    } else if a == 1 {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
