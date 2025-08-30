use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64
    }
    let mut a = Vec::new();
    for i in 0..10 {
        if i == 0 {
            a.push(x);
        } else if i == 1 {
            a.push(y);
        } else {
            a.push(f(a[i - 2] + a[i - 1]));
        }
    }
    println!("{}", a[9]);
}

fn f(a: u64) -> u64 {
    let mut degree = Vec::new();
    let mut b = a;
    while b > 0 {
        degree.push(b % 10);
        b /= 10;
    }
    degree.reverse();
    let mut num = 0;
    for i in 0..degree.len() {
        num += degree[i] * 10u64.pow(i as u32);
    }
    num
}