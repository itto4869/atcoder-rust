use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        image: [String; h],
    }
    println!("{}", "#".repeat(w + 2));
    for row in image {
        println!("#{}#", row);
    }
    println!("{}", "#".repeat(w + 2));
}
