use proconio::input;

fn main() {
    input! {
        sa: String,
        sb: String,
        sc: String,
    }
    let mut sa = sa.chars().collect::<Vec<_>>();
    let mut sb = sb.chars().collect::<Vec<_>>();
    let mut sc = sc.chars().collect::<Vec<_>>();
    let mut c = 'a';
    let mut s = &mut sa;
    while !s.is_empty() {
        c = s.remove(0);
        match c {
            'a' =>  s = &mut sa,
            'b' =>  s = &mut sb,
            'c' =>  s = &mut sc,
            _ => unreachable!("unexpected char: {c:?}"),
        }
    }
    println!("{}", c.to_uppercase().next().unwrap());
}
