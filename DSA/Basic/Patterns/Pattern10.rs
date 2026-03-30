fn main() {
    pattern10(5)
}

fn pattern10(n: i32) {
    let i_limit = n*2-1;
    let mut t = n - 1;

    for i in 1..=i_limit {
        if i <= n {
            for j in 1..=i {
                print!("*");
            }
        } else {
            for j in 1..=t {
                print!("*");
            }
            t -= 1;
        }
        println!();
    }
}