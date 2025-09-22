use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
    }
    
    let mut power = 1;
    for i in 0..n {
        power = power * (i + 1) % MOD;
    }

    println!("{}", power);
}
