fn main() {
    pattern19(5);
}

fn pattern19(n: i32) {
    let mut spaces = 0;

    for i in (1..=n).rev() {
        if i < n {spaces += 2;}

        for _ in 1..=i {print!("*");}
        for _ in 0..spaces {print!(" ");}
        for _ in 1..=i {print!("*");}

        println!();
    }        

    for i in 1..=n {
        for _ in 1..=i {print!("*");}
        for _ in 0..spaces {print!(" ");}
        for _ in 1..=i {print!("*");}

        spaces -= 2;
        println!();
    }
}