fn main() {
    println!("{}", count_odd_digits(12345));
}

fn count_odd_digits(n: i32) -> i32 {
    let mut count = 0;
    let mut t = n;

    while t > 0 {
        let digit = t%10;
        if digit%2 == 1 {count += 1;}
        t /= 10;
    }

    count
}