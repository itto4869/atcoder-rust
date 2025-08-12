use proconio::input;

fn main() {
    input! {
        mut n: u64,
        k: usize,
        a: [i64; k],
    }
    while iroha_hate(n, &a) {
        n += 1;
    }
    print!("{}", n);
}

fn iroha_hate(num: u64, a: &[i64]) -> bool {
    for n in num.to_string().chars() {
        let digit = n.to_digit(10).unwrap() as i64;
        if a.contains(&digit) {
            return true;
        }
    }
    false
}