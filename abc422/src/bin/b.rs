use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        grid: [String; h],
    }
    let grid = grid
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dist = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                continue;
            }
            let mut cnt = 0;
            for (dx, dy) in &dist {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                    continue;
                }
                if grid[ni as usize][nj as usize] == '#' {
                    cnt += 1;
                }
            }
            if cnt == 2 || cnt == 4 {
                continue;
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
