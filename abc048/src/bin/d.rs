use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    if s[0] == s[s.len() - 1] {
        if s.len() & 1 == 1 {
            println!("Second");
        } else {
            println!("First");
        }
    } else {
        if s.len() & 1 == 1 {
            println!("First");
        } else {
            println!("Second");
        }
    }
}
