use proconio::input;

fn main() {
    input! {
        a: char,
        b: char
    }
    match (a, b) {
        ('H', 'H') | ('D', 'D') => println!("H"),
        ('H', 'D') | ('D', 'H') => println!("D"),
        _ => unreachable!(),
    }
}
