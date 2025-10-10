use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut l: [u64; n],
    }
    l.sort();
    l.reverse();
    let mut ans = 0;
    for i in 0..k {
        ans += l[i];
    }
    println!("{}", ans);
}
