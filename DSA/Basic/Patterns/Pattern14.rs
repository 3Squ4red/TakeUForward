fn main() {
    pattern14(5)
}

fn pattern14(n: u8) {
    for i in 1..=n {
        for j in 65..65+i {
            print!("{}", j as char);
        }
        println!()
    }
}