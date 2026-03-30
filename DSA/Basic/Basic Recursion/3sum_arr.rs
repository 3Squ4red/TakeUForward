fn main() {
    println!("{}", array_sum(&[5, 8, 2], 3));
}

fn array_sum(a: &[i32], n: usize) -> i32 {
    if n == 1 {return a[0];}
    a[n-1]+array_sum(a, n-1)
}