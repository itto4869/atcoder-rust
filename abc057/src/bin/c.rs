use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = std::u64::MAX;
    for i in 1..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            let j = n / i;
            ans = ans.min(f(i, j));
        }
    }

    println!("{}", ans);
}

fn f(a: u64, b: u64) -> u64 {
    let a_digit = digit(a);
    let b_digit = digit(b);
    a_digit.max(b_digit)
}

fn digit(x: u64) -> u64 {
    let mut d = 0;
    let mut y = x;
    while y > 0 {
        d += 1;
        y /= 10;
    }
    d
}