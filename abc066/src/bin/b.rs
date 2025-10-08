use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes
    }
    let odd;
    let length = if s.len() % 2 == 0 {
        odd = false;
        s.len()
    } else {
        odd = true;
        s.len() - 1
    };
    if odd && s[0..length / 2] == s[length / 2..length] {
        println!("{}", length);
        return;
    }
    for i in 1..length / 2 {
        if s[0..length / 2 - i] == s[length / 2 - i..length - 2 * i] {
            println!("{}", length - 2 * i);
            return;
        }
    }
}
