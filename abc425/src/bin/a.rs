use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0i32;
    for i in 1..=n {
        if i % 2 == 1 {
            ans -= (i as i32).pow(3);
        } else {
            ans += (i as i32).pow(3);
        }
    }
    println!("{ans}");
}
