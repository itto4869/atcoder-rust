use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        a: [i64; n],
    }
    let mut max_difference = 0;
    let mut min_price = std::i64::MAX;
    let mut count = 0;
    for &price in &a {
        min_price = min_price.min(price);
        let difference = price - min_price;
        if difference == max_difference {
            count += 1;
        } else if difference > max_difference {
            max_difference = difference;
            count = 1;
        } else {
        }
    }
    println!("{}", count);
}
