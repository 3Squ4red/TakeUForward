fn main() {
    pattern8(5)
}

fn pattern8(n: i32) {
    let mut s = 2*n-1;

    for i in 0..n {
        for _ in 0..i {print!(" ");}
        for _ in 1..=s {print!("*");}
        println!();
        s -= 2;
    }
}
