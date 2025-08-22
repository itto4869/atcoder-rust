use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: i64, w: i64, n: usize,
        ab: [(i64, i64); n],
    }

    // 左上 (r0, c0) → その 3x3 に含まれる黒マス数
    let mut cnt: HashMap<(i64, i64), u8> = HashMap::with_capacity(n * 9 + 3);

    for &(a, b) in &ab {
        for dr in -2..=0 {
            for dc in -2..=0 {
                let r0 = a + dr;
                let c0 = b + dc;
                // 3x3 の左上が盤面内か（右下が h,w を越えないか）
                if r0 >= 1 && r0 + 2 <= h && c0 >= 1 && c0 + 2 <= w {
                    *cnt.entry((r0, c0)).or_insert(0) += 1;
                }
            }
        }
    }

    let mut ans = [0u64; 10];
    for &v in cnt.values() {
        ans[v as usize] += 1;
    }

    let total_3x3 = (h - 2) * (w - 2);         // すべての 3x3 の個数
    ans[0] = (total_3x3 - cnt.len() as i64) as u64;

    for j in 0..=9 {
        println!("{}", ans[j]);
    }
}
