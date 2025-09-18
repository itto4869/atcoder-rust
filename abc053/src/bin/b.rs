use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    let p1 = s.iter().position(|&c| c == 'A').unwrap();
    let p2 = s.iter().rposition(|&c| c == 'Z').unwrap();
    println!("{}", p2 - p1 + 1);
}
