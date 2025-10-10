use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut sum = 0;
    for &ai in &a {
        sum += ai;
    }
    let mut left_sum = 0;
    let mut ans = 1 << 60;
    for i in 0..n - 1 {
        left_sum += a[i];
        let right_sum = sum - left_sum;
        ans = ans.min((right_sum - left_sum).abs());
    }
    println!("{}", ans);
}
