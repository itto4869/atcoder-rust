use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [u64; n],
    }
    let mut left = r - 1;
    let mut right = r - 1;
    let mut count = 0u64;
    for i in 0..r {
        if l[i] == 0 {
            left = i;
            break;
        } 
    }

    for i in (r..n).rev() {
        if l[i] == 0 {
            right = i;
            break;
        }
    }

    for i in left..(right+1) {
        if l[i] == 0 {
            count += 1;
        }
        if l[i] == 1 {
            count += 2;
        }
    }
    if left == right && l[left] == 0 {
        count = 1;
    } else if left == right && l[left] == 1 {
        count = 0;
    } else {}
    println!("{}", count);
}
