use proconio::input;

fn main() {
    input! {
        x: u64,
    }
    let mut ans = (x / 11) * 2;
    let r = x % 11;
    if r > 6 {
        ans += 2;
    } else if r > 0 {
        ans += 1;
    } else {
        
    }
    println!("{}", ans);

}
