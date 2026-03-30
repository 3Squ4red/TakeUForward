fn main() {
    println!("{}", lcm(30,45));
}

// fn lcm(a: i32, b: i32) -> i32 {
//     if a == b {return a;}
    
//     let smaller;
//     let larger;
//     if a < b {
//         smaller = a;
//         larger = b;
//     } else {
//         smaller = b;
//         larger = a;
//     };

//     if larger%smaller == 0 {return larger;}

//     let mut i = 2;
//     loop {
//         let prod = larger*i;
//         if prod % smaller == 0 {return prod;}
//         i+=1;
//     }
// }

fn lcm(a: i32, b: i32) -> i32 {
    a*b/gcd(a,b)
}

fn gcd(a: i32, b: i32) -> i32 {
    return 
    if b == 0 {a} 
    else {gcd(b, a%b)}
}
