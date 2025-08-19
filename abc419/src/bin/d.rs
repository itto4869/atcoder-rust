use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }
    let mut vec_s = s.chars().collect::<Vec<_>>();
    let mut vec_t = t.chars().collect::<Vec<_>>();
    for _ in 0..m {
        input! {
            l: usize,
            m: usize,
        }
        let temp_s = vec_s[l - 1..m].to_vec();
        let temp_t = vec_t[l - 1..m].to_vec();
        vec_s[l - 1..m].copy_from_slice(&temp_t);
        vec_t[l - 1..m].copy_from_slice(&temp_s);
    }
    println!("{}", vec_s.iter().collect::<String>());
}
