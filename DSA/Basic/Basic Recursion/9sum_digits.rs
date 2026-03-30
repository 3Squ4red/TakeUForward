fn main() {
    println!("{}", add_digits(529));
}

fn add_digits(n: u32) -> u32 {
    let digit = n%10;
    if digit == n {return n;}
    let sum = digit + add_digits(n/10);
    if sum%10 != sum {
        add_digits(sum)
    } else {
        sum
    }
}