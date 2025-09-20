use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let mut counter = vec![0; n];
    let mut ans = Vec::new();
    for _ in 0..k {
        input! {
            a: usize,
            _: usize,
        }
        counter[a - 1] += 1;
        if counter[a - 1] == m {
            ans.push(a.to_string());
        }
    }
    
    if ans.is_empty() {

    } else {
        println!("{}", ans.join(" "));
    }
}
