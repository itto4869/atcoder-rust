use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut bag = Vec::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: i64,
            }
            bag.push(x);
        } else {
            bag.sort();
            let min_value = bag.first().unwrap();
            println!("{}", min_value);
            bag.remove(0);
        }
    }
}
