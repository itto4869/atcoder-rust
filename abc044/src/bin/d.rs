use proconio::input;
//TODO: 未正解
fn main() {
    input! {
        n: i64,
        s: i64,
    }
    if s > n { println!("-1"); return; }
    if n == s { println!("{}", n+1); return; }

    let mut b = 2;
    while b*b <= n {
        if f(&b, &n) == s {
            println!("{}", b);
            return;
        }
        b += 1;
    }

    let mut x = 1;
    let mut ans = -1;
    while x*x <= n && x <= s {
        let y = s - x;
        if (n - y) < 0 || (n - y) % x != 0 {
            x += 1;
            continue;
        }
        let b = (n - y) / x;
        if b <= x || b <= y || b <= 1 {
            x += 1;
            continue;
        }
        if ans == -1 { ans = b; }
        else { ans = ans.min(b); }
        x += 1;
    }
    println!("{}", ans);
}

fn f(b: &i64, n: &i64) -> i64 {
    let mut r = *n;
    let mut res = 0;
    while r > 0 {
        res += r % b;
        r /= b;
    }
    res
}