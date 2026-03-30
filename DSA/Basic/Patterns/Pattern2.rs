fn main() {
    pattern2(5);
}

pub fn pattern2(n: i32) {
    for i in 0..n {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
