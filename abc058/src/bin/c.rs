use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![std::u8::MAX; 26];
    for _ in 0..n {
        input! {
            s: String,
        }
        let mut count = vec![0; 26];
        let s = s.chars().collect::<Vec<char>>();
        for c in s {
            count[(c as u8 - b'a') as usize] += 1;
        }
        for i in 0..26 {
            ans[i] = ans[i].min(count[i]);
        }
    }
    let mut result = String::new();
    for i in 0..26 {
        for _ in 0..ans[i] {
            result.push((b'a' + i as u8) as char);
        }
    }
    println!("{}", result);
}