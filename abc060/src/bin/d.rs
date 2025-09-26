use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(u64, u64); n],
    }
    let mut items = vec![Vec::new(); 4];
    for (wi, vi) in &wv {
        items[(wi - wv[0].0) as usize].push(*vi);
    }
        
}


