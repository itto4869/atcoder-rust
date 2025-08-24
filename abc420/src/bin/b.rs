use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n]
    }
    let s_vec = s.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut points = vec![0; n];

    for i in 0..m {
        let mut count = vec![0; 2];
        for j in 0..n {
            if s_vec[j][i] == '0' {
                count[0] += 1;
            } else {
                count[1] += 1;
            }
        }
        if count[0] == count[1] {
            points.iter_mut().for_each(|x| *x += 1);
        } else if count[0] < count[1] {
            points.iter_mut().enumerate().for_each(|(j, x)| {
                if s_vec[j][i] == '0' {
                    *x += 1;
                }
            });
        } else {
            points.iter_mut().enumerate().for_each(|(j, x)| {
                if s_vec[j][i] == '1' {
                    *x += 1;
                }
            });
        }
    }
    let max_point = points.iter().max().unwrap();
    let mut ans = Vec::new();
    for (i, &point) in points.iter().enumerate() {
        if point == *max_point {
            ans.push((i + 1).to_string());
        }
    }
    println!("{}", ans.join(" "));
}
