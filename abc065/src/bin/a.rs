use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        b: i64,
    };
    if -a + b <= 0 {
        println!("delicious");
    } else if -a + b <= x {
        println!("safe");
    } else {
        println!("dangerous");
    }
}
