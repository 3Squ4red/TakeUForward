fn main() {
    println!("{}", gcd(30,45));
}

fn gcd(a: i32, b: i32) -> i32 {
    return 
    if b == 0 {a} 
    else {gcd(b, a%b)}
}
