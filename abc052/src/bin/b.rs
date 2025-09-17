use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    let mut x = 0;
    let mut ans = 0;
    let s = s.bytes();
    for c in s {
        if c == b'I' {
            x += 1;
        } else {
            x -= 1;
        }
        ans = ans.max(x);
    }
    println!("{}", ans);
}
