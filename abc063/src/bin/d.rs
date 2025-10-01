use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        mut h: [i64; n],
    };
    h.sort();
    h.reverse();
    let mut ok = h[0] / b;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut need = 0;
        for &hi in &h {
            let rem = hi - mid * b;
            if rem > 0 {
                need += (rem + a - b - 1) / (a - b);
            }
        }
        if need <= mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
