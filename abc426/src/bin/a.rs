use proconio::input;

fn main() {
    input! {
        x: String,
        y: String,
    };
    let mut num_x = 0;
    let mut num_y = 0;
    if x == "Ocelot" {
        num_x = 0;
    } else if x == "Serval" {
        num_x = 1;
    } else {
        num_x = 2;
    }

    if y == "Ocelot" {
        num_y = 0;
    } else if y == "Serval" {
        num_y = 1;
    } else {
        num_y = 2;
    }

    if num_x >= num_y {
        println!("Yes");
    } else {
        println!("No");
    }
}
