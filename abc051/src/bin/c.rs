use proconio::input;

fn main() {
    input! {
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
    }

    // sx, sy -> tx, ty
    if tx > sx {
        print!("{}", "R".repeat((tx - sx) as usize));
    } else {
        print!("{}", "L".repeat((sx - tx) as usize));
    }
    if ty > sy {
        print!("{}", "U".repeat((ty - sy) as usize));
    } else {
        print!("{}", "D".repeat((sy - ty) as usize));
    }

    // tx, ty -> sx, sy
    if tx > sx {
        print!("{}", "L".repeat((tx - sx) as usize));
    } else {
        print!("{}", "R".repeat((sx - tx) as usize));
    }
    if ty > sy {
        print!("{}", "D".repeat((ty - sy) as usize));
    } else {
        print!("{}", "U".repeat((sy - ty) as usize));
    }

    // sx, sy -> tx, ty
    if tx > sx {
        print!("L");
    } else {
        print!("R");
    }
    if ty > sy {
        print!("{}", "U".repeat((ty - sy + 1) as usize));
    } else {
        print!("{}", "D".repeat((sy - ty + 1) as usize));
    }
    if tx > sx {
        print!("{}", "R".repeat((tx - sx + 1) as usize));
    } else {
        print!("{}", "L".repeat((sx - tx + 1) as usize));
    }
    if ty > sy {
        print!("D");
    } else {
        print!("U");
    }

    // tx, ty -> sx, sy
    if tx > sx {
        print!("R");
    } else {
        print!("L");
    }
    if ty > sy {
        print!("{}", "D".repeat((ty - sy + 1) as usize));
    } else {
        print!("{}", "U".repeat((sy - ty + 1) as usize));
    }
    if tx > sx {
        print!("{}", "L".repeat((tx - sx + 1) as usize));
    } else {
        print!("{}", "R".repeat((sx - tx + 1) as usize));
    }
    if ty > sy {
        print!("U");
    } else {
        print!("D");
    }
    println!();
}
