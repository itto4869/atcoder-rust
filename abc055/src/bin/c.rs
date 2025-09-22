use proconio::input;

fn main() {
    input! {
        n: u64,
        mut m: u64,
    }
    
    let mut ans = 0;
    if n < m / 2 {
        ans += n;
        m -= n * 2;
        ans += m / 4;
    } else {
        ans += m / 2;
    }

    println!("{}", ans);
}
