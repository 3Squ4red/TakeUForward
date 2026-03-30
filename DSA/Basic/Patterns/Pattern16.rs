fn main() {
    pattern16(5)
}

fn pattern16(n: u8) {
    for i in 65..n+65 {
        for j in 65..=i {
            print!("{}", i as char);
        }
        println!()
    }
}

/* another solution */

// fn pattern16(n: i32) {
//     for i in 0..n {
//         for j in 0..=i {
//             print!("{}", (65_u8+i as u8) as char);
//         }
//         println!()
//     }
// }