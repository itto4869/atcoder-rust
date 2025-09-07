use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            mut a: i64, mut b: i64, mut c: i64,
        }
        let ans1 = f1(&a, &b, &c);
        let ans2 = f2(&a, &b, &c);
        println!("{}", ans1.max(ans2));
    }
}

fn f1(a: &i64, b: &i64, c: &i64) -> i64 {
    let mut a = *a;
    let mut b = *b;
    let mut c = *c;
    let mut ans = 0;
    let x = (a / 2).min(c);
    let y = (c / 2).min(a);
    if x < y {
        ans += y;
        a -= y;
        c -= 2 * y;
    } else {
        ans += x;
        a -= 2 * x;
        c -= x;
    }
    if a < 1 || b < 1 || c < 1 {
        return ans;
    } else {
        ans += a.min(b).min(c);
        return ans;
    }
}

fn f2(a: &i64, b: &i64, c: &i64) -> i64 {
    let mut a = *a;
    let mut b = *b;
    let mut c = *c;
    let mut ans = 0;
    let mut diff = 0;
    // まずABC
    if b <= a.min(c) {
        ans += b;
        diff = b;
        a -= diff;
        c -= diff;
        b = 0;
    } else if b > a.min(c) {
        ans += a.min(c);
        diff = a.min(c);
        b -= diff;
        a -= diff;
        c -= diff;
    }
    let x = (a / 2).min(c);
    let y = (c / 2).min(a);
    if x < y {
        ans += y;
        a -= y;
        c -= 2 * y;
    } else {
        ans += x;
        a -= 2 * x;
        c -= x;
    }
    ans
}