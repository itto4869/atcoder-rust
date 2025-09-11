use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let v = s.split(',').collect::<Vec<_>>();
    println!("{}", v.join(" "));
}
