use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }
    println!("{}{}{}", s1.chars().next().unwrap().to_ascii_uppercase(), s2.chars().next().unwrap().to_ascii_uppercase(), s3.chars().next().unwrap().to_ascii_uppercase());
}
