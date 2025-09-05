use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut a: [i64; n],
    }
    let mut ans = 0;
    for i in 1..n {
        let diff = (a[i] + a[i - 1] - x).max(0);
        if a[i] < diff {
            a[i - 1] -= diff - a[i];
            a[i] = 0;
        } else {
            a[i] -= diff;
        }
        ans += diff;
    }
    println!("{}", ans);
}
