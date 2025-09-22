use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let s = s.chars().collect::<Vec<_>>();
    for a in ['S', 'W'] {
        for b in ['S', 'W'] {
            let mut ans = vec![' '; n];
            ans[0] = a;
            ans[n - 1] = b;
            if s[0] == 'o' && ans[0] == 'S' {
                ans[1] = ans[n - 1];
            } else if s[0] == 'o' && ans[0] == 'W' {
                ans[1] = if ans[n - 1] == 'S' { 'W' } else { 'S' };
            } else if s[0] == 'x' && ans[0] == 'S' {
                ans[1] = if ans[n - 1] == 'S' { 'W' } else { 'S' };
            } else {
                ans[1] = ans[n - 1];
            }

            for i in 1..n - 2 {
                if s[i] == 'o' && ans[i] == 'S' {
                    ans[i + 1] = ans[i - 1];
                } else if s[i] == 'o' && ans[i] == 'W' {
                    ans[i + 1] = if ans[i - 1] == 'S' { 'W' } else { 'S' };
                } else if s[i] == 'x' && ans[i] == 'S' {
                    ans[i + 1] = if ans[i - 1] == 'S' { 'W' } else { 'S' };
                } else {
                    ans[i + 1] = ans[i - 1];
                }
            }

            if ((s[n - 1] == 'o' && ans[n - 1] == 'S' && ans[n - 2] == ans[0])
                || (s[n - 1] == 'o' && ans[n - 1] == 'W' && ans[n - 2] != ans[0])
                || (s[n - 1] == 'x' && ans[n - 1] == 'S' && ans[n - 2] != ans[0])
                || (s[n - 1] == 'x' && ans[n - 1] == 'W' && ans[n - 2] == ans[0]))
                && ((s[n - 2] == 'o' && ans[n - 2] == 'S' && ans[n - 3] == ans[n - 1])
                || (s[n - 2] == 'o' && ans[n - 2] == 'W' && ans[n - 3] != ans[n - 1])
                || (s[n - 2] == 'x' && ans[n - 2] == 'S' && ans[n - 3] != ans[n - 1])
                || (s[n - 2] == 'x' && ans[n - 2] == 'W' && ans[n - 3] == ans[n - 1]))
            {
                println!("{}", ans.iter().collect::<String>());
                return;
            }
        }
    }
    println!("-1");
}
