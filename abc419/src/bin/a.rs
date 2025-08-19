use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s == "red" {
        println!("SSS");
    }
    else if s == "blue" {
        println!("FFF");
    }
    else if s == "green" {
        println!("MMM");
    }
    else {
        println!("Unknown");
    }
}
