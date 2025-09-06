use proconio::input;

fn main() {
    input! {
        c: char,
    }
    if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
        println!("vowel");
    } else {
        println!("consonant");
    }
}
