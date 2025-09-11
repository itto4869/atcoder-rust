use proconio::input;

fn main() {
    input! {
        k: u64,
        s: u64,
    }
    let mut ans = 0;
    for x in 0..k+1 {
        for y in x..k+1 {
            for z in y..k+1 {
                if x + y + z == s {
                    ans = match (x, y, z) {
                        (a, b, c) if a == b && b ==c => ans + 1,
                        (a, b, c) if a == b || b == c || a == c => ans + 3,
                        _ => ans + 6,
                    };
                }
            }
        }
    }
    println!("{}", ans);
}
