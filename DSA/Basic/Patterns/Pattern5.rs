fn main() {
    pattern5(5)
}

fn pattern5(n: i32) {
    for i in (1..=n).rev() {
        for j in 1..=i {
            print!("*");
        }
        println!()
    }
}