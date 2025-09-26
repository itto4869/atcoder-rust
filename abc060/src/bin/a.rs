use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
        c: String,
    }
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let c = c.chars().collect::<Vec<_>>();
    if a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0] {
        println!("YES");
    } else {
        println!("NO");
    }
}
