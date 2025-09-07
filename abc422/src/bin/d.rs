use core::num;

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let length = 2u64.pow(n as u32);
    let diff = k % length;
    let mut num_seq = vec![k/length; length as usize];
    let mut ans = 0u64;
    if diff % 2 == 0 {
        num_seq[0] += diff / 2;
        num_seq[(length - 1) as usize] += diff / 2;
        ans = diff / 2;
    } else {
        num_seq[0] += diff / 2;
        num_seq[(length - 1) as usize] += diff / 2 + 1;
        ans = diff / 2 + 1;
    }
    println!("{}", ans);
    println!("{}", num_seq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
