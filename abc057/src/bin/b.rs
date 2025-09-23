use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    }
    for (a, b) in ab {
        let mut min_dist = std::i64::MAX;
        let mut checkpoint = 0;
        for (i, (c, d)) in cd.iter().enumerate() {
            let dist = (a - c).abs() + (b - d).abs();
            if dist < min_dist {
                min_dist = dist;
                checkpoint = i + 1;
            }
        }
        println!("{}", checkpoint);
    }
    }
