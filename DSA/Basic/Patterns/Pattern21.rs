fn main() {
    pattern21(5);
}

fn pattern21(n: i32) {
    for i in 0..n {
        print!("*");
        for j in 0..n-2 {
            if i == 0 || i == n-1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        if n>1 {println!("*");}
    }
}