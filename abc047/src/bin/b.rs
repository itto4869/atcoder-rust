use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
    }
    let mut grid = vec![vec![1u64; w]; h];
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
            a: u64,
        }
        match a {
            1 => {
                for i in 0..h {
                    for j in 0..x {
                        grid[i][j] = 0;
                    }
                }
            }
            2 => {
                for i in 0..h {
                    for j in x..w {
                        grid[i][j] = 0;
                    }
                }
            }
            3 => {
                for i in 0..y {
                    for j in 0..w {
                        grid[i][j] = 0;
                    }
                }
            }
            4 => {
                for i in y..h {
                    for j in 0..w {
                        grid[i][j] = 0;
                    }
                }
            }
            _ => {}
        }
    }
    let ans = grid.iter().flatten().sum::<u64>();
    println!("{}", ans);
}
