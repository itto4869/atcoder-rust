use proconio::input;

fn main() {
    input! {
        h: isize,
        w: isize,
    }
    let mut base = w;
    let mut ans = std::i64::MAX;
    for i in 0..h {
        let block1 = base * (i + 1);
        if h - i - 2 > 0 {
            if (h - i - 1) % 2 == 0 {
                let block2 = base * (h - i - 1) / 2;
                let max_block = block1.max(block2);
                let min_block = block1.min(block2);
                ans = ans.min((max_block - min_block) as i64);
            } else {
                let block2 = base * (h - i - 1) / 2;
                let block3 = base * (h - i - 1) / 2 + w;
                let max_block = block1.max(block2).max(block3);
                let min_block = block1.min(block2).min(block3);
                ans = ans.min((max_block - min_block) as i64);
            }
        }
        if h - i - 1 > 0 {
            if w % 2 == 0 {
                let block2 = (w / 2) * (h - i - 1);
                let max_block = block1.max(block2);
                let min_block = block1.min(block2);
                ans = ans.min((max_block - min_block) as i64);
            } else {
                let block2 = (w / 2) * (h - i - 1);
                let block3 = (w / 2 + 1) * (h - i - 1);
                let max_block = block1.max(block2).max(block3);
                let min_block = block1.min(block2).min(block3);
                ans = ans.min((max_block - min_block) as i64);
            }
        }
    }

    base = h;
    for i in 0..w {
        let block1 = base * (i + 1);
        if w - i - 2 > 0 {
            if (w - i - 1) % 2 == 0 {
                let block2 = base * (w - i - 1) / 2;
                let max_block = block1.max(block2);
                let min_block = block1.min(block2);
                ans = ans.min((max_block - min_block) as i64);
            } else {
                let block2 = base * (w - i - 1) / 2;
                let block3 = base * (w - i - 1) / 2 + h;
                let max_block = block1.max(block2).max(block3);
                let min_block = block1.min(block2).min(block3);
                ans = ans.min((max_block - min_block) as i64);
            }
        }
        if w - i - 1 > 0 {
            if h % 2 == 0 {
                let block2 = (h / 2) * (w - i - 1);
                let max_block = block1.max(block2);
                let min_block = block1.min(block2);
                ans = ans.min((max_block - min_block) as i64);
            } else {
                let block2 = (h / 2) * (w - i - 1);
                let block3 = (h / 2 + 1) * (w - i - 1);
                let max_block = block1.max(block2).max(block3);
                let min_block = block1.min(block2).min(block3);
                ans = ans.min((max_block - min_block) as i64);
            }
        }
    }
    println!("{ans}");
}
