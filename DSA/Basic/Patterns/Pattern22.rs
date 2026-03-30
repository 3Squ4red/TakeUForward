fn main() {
    pattern22(5);
}

fn pattern22(n: i32) {
    let last = 2*n-1;

    for i in 0..last {
        for j in 0..last {
            let v = vec![i, last-j-1, last-i-1, j];

            print!("{} ", n-v.iter().min().unwrap());
        }

        println!();
    }
}