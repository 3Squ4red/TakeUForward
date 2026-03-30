fn main() {
    print!("{}", largest_digit(4655549));
}

fn largest_digit(i: i32) -> i32 {
    let mut max = 0;
    let mut t = i;

    while t > 0 {
        let digit = t%10;
        if digit > max {max = digit;}
        t /= 10;
    }

    max
}