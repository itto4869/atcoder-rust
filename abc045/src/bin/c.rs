use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ans = 0;
    for i in 1..n {
        let it = (0..n-1).combinations(i);
        for comb in it {
            for j in 0..comb.len() {
                if j == 0 {
                    let x = s[0..comb[j]+1].iter().collect::<String>().parse::<u64>().unwrap();
                    ans += x;
                }
                if j == comb.len() - 1 {
                    let x = s[comb[j]+1..n].iter().collect::<String>().parse::<u64>().unwrap();
                    ans += x;
                } else {
                    let x = s[comb[j]+1..comb[j+1]+1].iter().collect::<String>().parse::<u64>().unwrap();
                    ans += x;
                }
            }
        }
    }
    let x = s.iter().collect::<String>().parse::<u64>().unwrap();
    ans += x;
    println!("{ans}");
}
