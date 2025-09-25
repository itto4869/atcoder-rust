use proconio::input;

fn main() {
    input! {
        o: String,
        e: String
    }
    let o = o.chars().collect::<Vec<char>>();
    let e = e.chars().collect::<Vec<char>>();
    let mut ans = String::new();
    for i in 0..o.len().max(e.len()) {
        if i < o.len() {
            ans.push(o[i]);
        }
        if i < e.len() {
            ans.push(e[i]);
        }
    }
    println!("{}", ans);
}
