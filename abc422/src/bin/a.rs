use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    if s[2] == '8' {
        print!("{}-1", s[0].to_digit(10).unwrap() + 1);
    } else {
        print!("{}-{}", s[0], s[2].to_digit(10).unwrap() + 1);
    }
    println!();
}
