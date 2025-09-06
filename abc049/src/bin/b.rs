use proconio::input;

fn main() {
    input! {
        h: usize,
        _: usize,
        image: [String; h]
    }
    for line in image {
        println!("{}", line);
        println!("{}", line);
    }
}
