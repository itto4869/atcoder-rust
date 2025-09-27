use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut p = vec![0; n];
    let mut seen = HashSet::new();
    for i in 0..n {
        if seen.contains(&a[i]) {
            ///println!("No");
            return;
        }
        if a[i] != -1 {
            seen.insert(a[i]);
            p[i] = a[i];
        }
    }
    let mut unused: Vec<i64> = vec![];
    for i in 1..=n {
        if !seen.contains(&(i as i64)) {
            unused.push(i as i64);
        }
    }
    let mut j = 0;
    for i in 0..n {
        if p[i] == 0 {
            p[i] = unused[j];
            j += 1;
        }
    }
    
}
