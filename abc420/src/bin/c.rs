use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    let mut min_sum = 0;
    for i in 0..n {
        min_sum += min(a[i], b[i]);
    }

    for _ in 0..q {
        input! {
            c: char,
            x: i64,
            v: i64,
        }
        let diff = match c {
            'A' => {
                let temp = a[(x - 1) as usize];
                a[(x - 1) as usize] = v;
                min(v, b[(x - 1) as usize]) - min(temp, b[(x - 1) as usize])
            }
            'B' => {
                let temp = b[(x - 1) as usize];
                b[(x - 1) as usize] = v;
                min(a[(x - 1) as usize], v) - min(a[(x - 1) as usize], temp)
            }
            _ => unreachable!()
        };
        min_sum += diff;
        println!("{}", min_sum);
    }
}
