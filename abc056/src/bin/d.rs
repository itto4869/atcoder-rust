use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    }
    a.sort();
    let mut idx = 0;
    let mut sum = 0;
    let mut sums = Vec::new();
    while sum < k && idx < n {
        sum += a[idx];
        sums.push(sum);
        idx += 1;
    }

    if sum < k {
        println!("{}", n);
        return;
    }

    let mut ans = 0;
    for i in 0..idx {
        if sum - a[i] >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
