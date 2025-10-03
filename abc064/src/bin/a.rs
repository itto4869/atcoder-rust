use proconio::input;

fn main() {
    input! {
        r: u64,
        g: u64,
        b: u64,
    };
    let num = r * 100 + g * 10 + b;
    if num % 4 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
