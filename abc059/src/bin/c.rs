use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let ans = calc_cost(&a, true).min(calc_cost(&a, false));
    println!("{ans}");
}

fn calc_cost(a: &[i64], start_positive: bool) -> i64 {
    let mut ans = 0;
    let mut sum = 0;
    let mut positive = start_positive;
    for &x in a {
        sum += x;
        if positive {
            if sum <= 0 {
                ans += 1 - sum;
                sum = 1;
            }
        } else {
            if sum >= 0 {
                ans += sum + 1;
                sum = -1;
            }
        }
        positive = !positive;
    }
    ans
}