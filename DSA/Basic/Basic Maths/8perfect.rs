fn main() {
    println!("{}", perfect(1));
}

// fn perfect(n: u32) -> bool {
//     let mut v = vec![1];

//     for i in 2..=n/2 {
//         if n%i == 0 {v.push(i);}
//     }

//     n == v.iter().sum()
// }

fn perfect(n: u32) -> bool {
    if n < 2 {return false;}
    let mut sum = 1;

    for i in 2..=n/2 {
        if n%i == 0 {sum += i;}
    }

    sum == n
}
