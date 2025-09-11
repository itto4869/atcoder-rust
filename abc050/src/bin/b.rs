use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u64; n],
        m: usize,
    }
    let ans = t.iter().sum::<u64>();
    for _ in 0..m {
        input! {
            p: usize,
            x: u64,
        }
        let res = ans - t[p - 1] + x;
        println!("{res}");
    }
}
