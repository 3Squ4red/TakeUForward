fn main() {
    pattern7(5)
}

fn pattern7(n: i32) {
    let mut s = 1;

    for i in (0..n).rev() {
        for _ in 1..=i {print!(" ");}
        for _ in 1..=s {print!("*");}
        println!();
        s += 2;
    }
}
