fn main() {
    println!("{}", prime_upto_n(20));
}

fn prime_upto_n(i: i32) -> i32 {
    let mut count = 0;

    for i in 0..=i {
        if is_prime(i) {count+=1;}
    }

    count
}

fn is_prime(n: i32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    // Using f64 for the square root calculation
    let limit = (n as f64).sqrt() as i32;
    
    // We only check odd numbers starting from 3
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}