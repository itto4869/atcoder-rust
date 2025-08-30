use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut s = Vec::new();
    for _ in 0..n {
        input! {
            name: String,
        }
        s.push(name);
    }
    input! {
        x: usize,
        y: String,
    }
    if s[x - 1] == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
