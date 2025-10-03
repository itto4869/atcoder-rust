use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = String::new();
    let mut left = 0;
    let mut right = 0;
    for c in s {
        if left > 0 && c == ')' {
            left -= 1;
            ans += ")";
        } else if c == '(' {
            left += 1;
            ans += "(";
        } else {
            right += 1;
        }
        println!("{} {} {}", left, right, ans);
    }
    for _ in 0..right {
        ans = "(".to_string() + &ans;
    }
    for _ in 0..left {
        ans = "(".to_string() + &ans;
    }
    for _ in 0..left {
        ans += ")";
    }
    println!("{}", ans);
}
