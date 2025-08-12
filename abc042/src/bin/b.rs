use proconio::input;

fn main() {
    input! {
        n: usize,
        _l: usize,
        mut a: [String; n],
    }

    a.sort();
    for s in a {
        print!("{}", s);
    }
}
