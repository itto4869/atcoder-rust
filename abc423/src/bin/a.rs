use proconio::input;

fn main() {
    input! {
        x: i64,
        c: i64,
    }
    for i in 0..(x / 1000) {
        if 1000 * i + i * c > x {
            println!("{}", 1000 * (i - 1));
            return;
        }
    } 
    println!("{}", 0);
}
