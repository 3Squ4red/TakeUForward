fn main() {
    print!("{}\n", reverse_number(123));
}

// fn reverse_number(i: i32) -> i32 {
//     i.to_string().chars().rev().collect::<String>().parse().unwrap()
// }

// fn reverse_number(n: i32) -> i32 {
//     let mut res = 0;
//     let mut t = n;
//     let mut i = n.to_string().len() as u32;

//     while t > 0 {
//         res += (t%10) * 10_i32.pow(i-1);
//         t /= 10;
//         i-=1;
//     }

//     res
// }

fn reverse_number(i: i32) -> i32 {
    let mut reversed = 0;
    let mut temp = i;

    while temp != 0 {
        reversed = (reversed * 10) + temp % 10;
        temp /= 10;
    }

    return reversed;
}