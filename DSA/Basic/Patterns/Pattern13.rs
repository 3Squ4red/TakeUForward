fn main() {
    pattern13(5)
}

fn pattern13(n: i32) {
    let mut k = 1;
    for i in 1..=n {
        for j in 1..=i {
            print!("{} ", k);
            k+=1;
        }
        println!()
    }
}