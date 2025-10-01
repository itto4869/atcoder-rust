use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = 0;
    let mut points = Vec::new();
    for _ in 0..n {
        input! {
            s: u64,
        };
        sum += s;
        points.push(s);
    }
    if sum % 10 != 0 {
        println!("{}", sum);
        return;
    } else {
        points.sort();
        for p in points {
            if p % 10 != 0 {
                println!("{}", sum - p);
                return;
            }
        }
        println!("0");
    }
}
