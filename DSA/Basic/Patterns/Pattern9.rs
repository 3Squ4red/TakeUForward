fn main() {
    pattern9(5)
}

fn pattern9(n: i32) {
    let mut s = 1;

    for i in (0..n).rev() {
        for _ in 1..=i {print!(" ");}
        for _ in 1..=s {print!("*");}
        println!();
        s += 2;
    }

    let mut t = 2*n-1;

    for i in 0..n {
        for _ in 0..i {print!(" ");}
        for _ in 1..=t {print!("*");}
        println!();
        t -= 2;
    }
}
