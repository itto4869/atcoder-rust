use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [String; n],
        b: [String; m],
    }
    let a = a.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let b = b.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut ok = false;
    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            ok = true;
            for k in 0..m {
                for l in 0..m {
                    if a[i + k][j + l] != b[k][l] {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    match ok {
        true => println!("Yes"),
        false => println!("No"),
    }
}
