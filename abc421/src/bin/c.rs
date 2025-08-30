use proconio::input;
use std::mem::swap;
use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let mut s = s.chars().collect::<Vec<_>>();
    let mut count = 0u64;
    let mut c = s[0];
    let mut not_c = if c == 'A' { 'B' } else { 'A' };
    let mut a_stock = VecDeque::new();
    let mut b_stock = VecDeque::new();
    for i in 0..s.len() {
        if s[i] != c {
            if !a_stock.is_empty() && s[i] == 'B' {
                let idx = a_stock.pop_front().unwrap();
                count += (i - idx) as u64;
                s[i] = 'A';
            } else if !b_stock.is_empty() && s[i] == 'A' {
                let idx = b_stock.pop_front().unwrap();
                count += (i - idx) as u64;
                s[i] = 'B';
            } else {
                match s[i] {
                'A' => {
                    a_stock.push_back(i);
                },
                'B' => {
                    b_stock.push_back(i);
                },
                _ => unreachable!(),
            }
            }
        } 
        swap(&mut c, &mut not_c);

    }
    println!("{}", count);
}
