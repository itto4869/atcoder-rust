use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut min_version = 1;
    let mut pc = vec![1u64; n+1];
    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }
        if x < min_version {
            println!("0");
        } else {
            let mut res = 0;
            for i in min_version..=x {
                res += pc[i];
                pc[i] = 0;
            }
            min_version = x;
            pc[y] += res;
            println!("{}", res);
        }
    }
}
