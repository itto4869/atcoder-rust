use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut result = String::new();
    for c in s.chars() {
        if c == '0' {
            result.push('0');
        }
        else if c == '1' {
            result.push('1');
        }
        else {
            result.pop();
        }
    }
    print!("{}", result);
}