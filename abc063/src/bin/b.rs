use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    };
    let c: HashSet<char> = s.chars().collect();
    if s.chars().count() == c.len() {
        println!("yes");
    } else {
        println!("no");
    }
}
