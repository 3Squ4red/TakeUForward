fn main() {
    pattern15(5)
}

fn pattern15(n: u8) {
    for i in (1..=n).rev() {
        for j in 65..65+i {
            print!("{}", j as char);
        }
        println!()
    }
}