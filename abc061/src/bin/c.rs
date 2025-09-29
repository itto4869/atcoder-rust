use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    
    let mut map = BTreeMap::new();
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        *map.entry(a).or_insert(0) += b;
    }

    let mut cnt = 0;
    for (&key, &val) in &map {
        cnt += val;
        if cnt >= k as u64 {
            println!("{}", key);
            return;
        }
    }
}
