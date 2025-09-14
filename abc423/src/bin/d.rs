use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: i128,
    }

    let mut customers: BTreeMap<i128, i128> = BTreeMap::new();
    let mut cnt = 0i128;
    let mut time = 0i128;
    for _ in 0..n {
        input! {
            a: i128,
            b: i128, 
            c: i128,
        }
        while let Some((&t, &num)) = customers.iter().next() {
            if t <= a {
                customers.remove(&t);
                cnt -= num;
                time = t;
            } else {
                break;
            }
        }
        if customers.len() == 0 && time == 0 {
            customers.insert(a + b, c);
            time = a;
            cnt = c;
        } else if cnt + c <= k {
            time = time.max(a);
            customers.insert(time + b, c);
            cnt += c;
        }
        else {
            while let Some((&t, &num)) = customers.iter().next() {
                if cnt + c - num <= k {
                    customers.remove(&t);
                    cnt -= num;
                    time = t;
                    customers.insert(t + b, c);
                    cnt += c;
                    break;
                } else {
                    cnt -= num;
                    customers.remove(&t);
                    time = t;
                }
            }
        }
        println!("{}", time);
    }
}
