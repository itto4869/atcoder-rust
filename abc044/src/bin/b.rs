use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        w: String,
    }
    let mut map = HashMap::new();
    for c in w.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    for (_, count) in map {
        if count % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
