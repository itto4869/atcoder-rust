use proconio::input;

fn main() {
    input! {
        mut s: [i64; 3],
    }
    s.sort();
    if s == [5, 5, 7] {
        print!("YES");
    }
    else {
        print!("NO");
    }
}
