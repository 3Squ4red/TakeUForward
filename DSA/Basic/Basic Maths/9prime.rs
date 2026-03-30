fn main() {
    println!("{}", is_prime(6));
}

// fn is_prime(n: i32) -> bool {
//     if n < 2 {return false;}

//     for i in 2..=n/2 {
//         if n%i == 0 {return false;}
//     }

//     true
// }

// AI optimized solution
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