fn main() {
    println!("{}", fib(6));
}

fn fib(n: u32) -> u32 {
    if n<2 {return n;}
    fib(n-1) + fib(n-2)
}