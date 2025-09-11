use proconio::input;
use std::collections::HashMap;
use ac_library::ModInt1000000007 as Mint;

fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            a: u64,
        }
        let count = map.entry(a).or_insert(0);
        *count += 1;
    }
    
    let mut vec = map.iter().collect::<Vec<(&u64, &u64)>>();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..vec.len() {
        if *vec[i].0 == 0 && *vec[i].1 > 1 {
            println!("0");
            return;
        } else if *vec[i].1 > 2 {
            println!("0");
            return;
        } else if i > 0 && *vec[i].0 - *vec[i - 1].0 != 2 {
            println!("0");
            return;
        } else { continue; }
    }


    let ans = Mint::new(2).pow(n as u64 / 2);
    println!("{ans}");
    
}