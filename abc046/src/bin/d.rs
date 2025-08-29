use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s_bytes = s.as_bytes();
    let mut p_count = 0;
    let mut g_count = 0;
    let mut result = Vec::with_capacity(s.len());
    for &b in s_bytes {
        if p_count == g_count {
            g_count += 1;
            match b {
                b'p' => {
                    result.push(-1);
                }
                b'g' => {
                    result.push(0);
                }
                _ => unreachable!(),
            }
        }
        else {
            match b {
                b'p' => {
                    p_count += 1;
                    result.push(0);
                }
                b'g' => {
                    p_count += 1;
                    result.push(1);
                }
                _ => unreachable!(),
            }
        }
    }
    let ans = result.iter().sum::<i32>();
    println!("{}", ans);
}
