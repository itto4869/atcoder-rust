use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut result = (-1, -1);
    let char_s = s.chars().collect::<Vec<_>>();
    for i in 0..(s.len()-1) {
        if char_s[i] == char_s[i+1] {
            result = ((i+1) as i64, (i+2) as i64);
            break;
        }
        else if i < (s.len()-2) && char_s[i] == char_s[i+2] {
            result = ((i+1) as i64, (i+3) as i64);
            break;
        }
    }
    print!("{} {}", result.0, result.1);
}
