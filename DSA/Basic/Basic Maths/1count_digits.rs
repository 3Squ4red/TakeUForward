fn main() {
    println!("{}", count_digits(12345));
}

// fn count_digits(n: i32) -> usize {
//     n.to_string().chars().count()
// }

fn count_digits(n: i32) -> i32 {
    let mut count = 0;
    let mut t = n;

    while t > 0 {
        t /= 10;
        count += 1;
    }

    count
}