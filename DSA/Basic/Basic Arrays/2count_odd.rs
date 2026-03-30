fn main() {
    println!("{}", count_odd(&[1, 2, 34, 11, 4, 5], 6));
}

fn count_odd(a: &[i32], n: usize) -> u32 {
    let mut c = 0;
    for n in a {
        // if n % 2 == 1 {
        //     c += 1;
        // }
        if ((n & 1) == 1) { // Checks if the last bit is 1
            c+=1;
        }
    }
    c
}
