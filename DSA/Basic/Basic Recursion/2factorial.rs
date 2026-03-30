fn main() {
    println!("{}", factorial(5));
}

fn factorial(n: u32) -> u32 {
    if n < 2 {return 1;}
    n*factorial(n-1)
}
