fn main() {
    println!("{}", is_armstrong(370));
}

fn is_armstrong(n: u32) -> bool {
    let l = n.to_string().len();
    let mut sum = 0;
    let mut t = n;

    while t > 0 {
        sum += (t%10).pow(l as u32);
        t /= 10;
    }

    sum == n
} 