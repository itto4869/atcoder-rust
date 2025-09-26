use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }
    let num = a % b;
    for i in 0..b {
        if (num * i) % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
