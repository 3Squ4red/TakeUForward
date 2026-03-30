fn main() {
    println!("{}", nnumbers_sum(0));
}

fn nnumbers_sum(n: u32) -> u32 {
    if n < 2 {return n;}
    n+nnumbers_sum(n-1)
}