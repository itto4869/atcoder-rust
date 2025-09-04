use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
