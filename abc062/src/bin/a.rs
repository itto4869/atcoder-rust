use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
    }
    if x == 2 && y == 2 {
        println!("Yes");
    } else if [1, 3, 5, 7, 8, 10, 12].contains(&x) && [1, 3, 5, 7, 8, 10, 12].contains(&y) {
        println!("Yes");
    } else if [4, 6, 9, 11].contains(&x) && [4, 6, 9, 11].contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
