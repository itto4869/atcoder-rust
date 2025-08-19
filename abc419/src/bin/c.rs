use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut min_x_p = (1_000_000_000, 1_000_000_000);
    let mut min_y_p = (1_000_000_000, 1_000_000_000);
    let mut max_x_p = (1, 1);
    let mut max_y_p = (1, 1);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        if x < min_x_p.0 {
            min_x_p = (x, y);
        }
        if y < min_y_p.1 {
            min_y_p = (x, y);
        }
        if x > max_x_p.0 {
            max_x_p = (x, y);
        }
        if y > max_y_p.1 {
            max_y_p = (x, y);
        }
    }

    let center = ((min_x_p.0 + max_x_p.0) / 2, (min_y_p.1 + max_y_p.1) / 2);
    let mut dists = [[0; 2]; 4];
    for (i, (x, y)) in [min_x_p, min_y_p, max_x_p, max_y_p].iter().enumerate() {
        let dx = (x - center.0).abs();
        let dy = (y - center.1).abs();
        dists[i][0] = dx;
        dists[i][1] = dy;
    }
    let max_dist_p = dists.iter().max_by_key(|&d| d[0] + d[1]).unwrap();
    let mut ans = 0;
    if max_dist_p[0].abs() > max_dist_p[1].abs() {
        ans = max_dist_p[1].abs() + max_dist_p[0].abs() - max_dist_p[1].abs();
    } else {
        ans = max_dist_p[1].abs();
    }
    println!("{}", ans);
}
