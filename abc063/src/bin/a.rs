use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64
    };
    let ans = a + b;
    if ans >= 10 {
        println!("error");
    } else {
        println!("{}", ans);
    }
}
