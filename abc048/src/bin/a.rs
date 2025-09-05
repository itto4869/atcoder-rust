use proconio::input;

fn main() {
    input! {
        _: String,
        x: String,
        _: String,
    }
    let x = x.chars().collect::<Vec<char>>();
    println!("A{}C", x[0]);
}
