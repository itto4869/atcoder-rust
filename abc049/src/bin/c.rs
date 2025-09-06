use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    let pattern = ["eraser", "erase", "dreamer", "dream"];
    while !s.is_empty() {
        for p in pattern {
            if let Some(rest) = s.strip_suffix(p) {
                s = rest.to_string();
                break;
            }
            if p == "dream" {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
