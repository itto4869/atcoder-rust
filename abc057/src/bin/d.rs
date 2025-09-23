use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut v: [i128; n],
    }
    v.sort();
    v.reverse();

    let mut cnt = 0u128;
    let mut max = 0.0;
    let mut sum: i128 = v.iter().take(a - 1).sum();
    for i in a..b + 1 {
        sum += v[i - 1];
        let avg = sum as f64 / i as f64;
        if max == avg {
            cnt += 1;
        } else if max < avg {
            max = avg;
            cnt = 1;
        }
    }
    if v[0] as f64 == max {
        let value_cnt = v.iter().filter(|&&x| x == v[0]).count();
        cnt = 0;
        for i in a..=value_cnt.min(b) {
            cnt += combination(value_cnt, i);
        }
    }
    println!("{:.6}", max);
    println!("{}", cnt);
}

fn combination(n: usize, r: usize) -> u128 {
    if r > n {
        return 0;
    }
    let mut result = 1u128;
    let r = r.min(n - r);
    for i in 0..r {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }
    result
}
