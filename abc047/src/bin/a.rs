use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let mut abc = [a, b, c];
    abc.sort();
    if abc[0] + abc[1] == abc[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
