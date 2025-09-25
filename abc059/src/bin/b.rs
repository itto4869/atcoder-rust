use proconio::input;
use num_bigint::BigInt;

fn main() {
    input! {
        a: BigInt,
        b: BigInt,
    }
    if a > b {
        println!("GREATER");
    } else if a < b {
        println!("LESS");
    } else {
        println!("EQUAL");
    }
}
