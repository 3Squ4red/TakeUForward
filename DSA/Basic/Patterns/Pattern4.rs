fn main() {
    pattern4(5)
}

fn pattern4(n: i32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("{}", i);
        }
        println!()
    }
}