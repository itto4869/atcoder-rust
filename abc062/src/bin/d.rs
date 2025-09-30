use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        a: [u64; 3*n],
    }
    let mut heap = BinaryHeap::new();
    let mut left = vec![0; 2*n + 1];
    for i in 0..n {
        heap.push(Reverse(a[i]));
        left[i + 1] = left[i] + a[i];
    }
    for i in n..2*n {
        let min = heap.peek().unwrap().0;
        if a[i] > min {
            left[i + 1] = left[i] + a[i] - min;
            heap.pop();
            heap.push(Reverse(a[i]));
        } else {
            left[i + 1] = left[i];
        }
    }

    let mut right = vec![0; 2*n + 1];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(a[3*n - 1 - i]);
        right[n - i - 1] = right[n - i] + a[3*n - 1 - i];
    }
    for i in n..2*n {
        let max = heap.peek().unwrap();
        if a[2*n - 1 - i] < *max {
            right[n - i - 1] = right[n - i] + a[2*n - 1 - i] - *max;
            heap.pop();
            heap.push(a[2*n - 1 - i]);
        } else {
            right[n - i - 1] = right[n - i];
        }
    }
}