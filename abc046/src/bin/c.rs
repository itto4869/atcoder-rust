use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(i64, i64); n],
    }
    let (mut x, mut y) = (1i64, 1i64);
    for (t, a) in a {
        if x <= t && y <= a {
            x = t;
            y = a;
        } else {
            let p = x / t;
            let q = y / a;
            if p > q && x % t != 0 {
                x = t * (p + 1);
                y = a * (p + 1);
            }
            else if p > q && x % t == 0 {
                x = t * p;
                y = a * p;
            }
            else if p < q && y % a != 0 {
                x = t * (q + 1);
                y = a * (q + 1);
            }
            else if p < q && y % a == 0 {
                x = t * q;
                y = a * q;
            }
            else {
                if x % t != 0 || y % a != 0 {
                    x = t * (p + 1);
                    y = a * (q + 1);
                }
                else {
                    x = t * p;
                    y = a * q;
                }
            }
        }
    }
    println!("{}", x + y);
}
