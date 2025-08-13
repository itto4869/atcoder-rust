use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut costs = [0; 201];
    for i in -100..101 {
        costs[(i + 100) as usize] = cost(&a, i);
    }
    let min_cost = costs.iter().min().unwrap();
    print!("{}", min_cost);
}

fn cost(a: &[i64], y: i64) -> i64 {
    a.iter()
        .map(|&a_i| (a_i - y).pow(2))
        .sum()
}