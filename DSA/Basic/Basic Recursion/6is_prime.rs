fn main() {
    println!("{}", is_prime(3));
}

fn is_prime(n: i32) -> bool {
    if n == 2 || n == 3 {return true;}
    if n < 2 || n&1 == 0 {return false;}
    
    helper(n, 3, ((n as f32).sqrt()) as i32)
}

fn helper(n: i32, i: i32, sqrt: i32) -> bool {
    let divisible = n%i == 0;

    if i >= sqrt {return !divisible;}
    else {!divisible && helper(n, i+2, sqrt)}
}
