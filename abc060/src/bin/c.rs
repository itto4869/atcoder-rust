use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        time: [u64; n],
    }
    let mut ans =0;
    let mut current = 0;
    for ti in time {
        if current <= ti {
            ans += t;
            current = ti + t;
        } else {
            ans += (ti + t) - current;
            current = ti + t;
        }
    }
    println!("{}", ans);
}
