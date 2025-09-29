use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ans: Vec<u64> = vec![0; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        ans[a - 1] += 1;
        ans[b - 1] += 1;
    }

    for v in ans {
        println!("{}", v);
    }
}
