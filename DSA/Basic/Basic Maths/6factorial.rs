fn main() {
    println!("{}", factorial(5));
}

fn factorial(i: u32) -> u32 {
    return if i == 1 {1} else {i * factorial(i-1)}
}