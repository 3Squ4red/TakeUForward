fn main() {
    pattern6(5)
}

fn pattern6(n: i32) {
    for i in (1..=n).rev() {
        for j in 1..=i {
            print!("{}", j);
        }
        println!()
    }
}