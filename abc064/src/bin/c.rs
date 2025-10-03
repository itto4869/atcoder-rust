use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut colors = HashSet::new();
    let mut any_color_count = 0;
    for &ai in &a {
        let c = color(ai);
        if c == "any color" {
            any_color_count += 1;
        } else {
            colors.insert(c);
        }
    }
    let min_colors = colors.len().max(1);
    let max_colors = colors.len() + any_color_count;
    println!("{} {}", min_colors, max_colors);
}

fn color(num: u64) -> &'static str {
    if 1 <= num && num <= 399 {
        "gray"
    } else if 400 <= num && num <= 799 {
        "brown"
    } else if 800 <= num && num <= 1199 {
        "green"
    } else if 1200 <= num && num <= 1599 {
        "light blue"
    } else if 1600 <= num && num <= 1999 {
        "blue"
    } else if 2000 <= num && num <= 2399 {
        "yellow"
    } else if 2400 <= num && num <= 2799 {
        "orange"
    } else if 2800 <= num && num <= 3199 {
        "red"
    } else {
        "any color"
    }
}