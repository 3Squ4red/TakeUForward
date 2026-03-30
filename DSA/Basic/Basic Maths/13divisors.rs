fn main() {
    println!("{:?}", divisors(20));
}

fn divisors(n: i32) -> Vec<i32> {
    let mut v = vec![1];
    for i in 2..=n/2 {
        if n%i == 0 {v.push(i);}
    }
    v.push(n);
    v
}