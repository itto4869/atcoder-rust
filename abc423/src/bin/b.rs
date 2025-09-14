use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }
    let mut reachable1 = 0i64;
    let mut reachable2 = 0i64;
    for (i, &li) in l.iter().enumerate() {
        if li == 1 {
            reachable1 = i as i64;
            break;
        }
    };
    for (i, &li) in l.iter().enumerate().rev() {
        if li == 1 {
            reachable2 = i as i64;
            break;
        }
    };
    println!("{}", reachable2 - reachable1);
}
