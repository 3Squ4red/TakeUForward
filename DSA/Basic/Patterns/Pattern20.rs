fn main() {
    pattern20(5)
}

fn pattern20(n: i32) {
    let i_limit = n*2-1;
    let mut t = n - 1;
    let mut spaces = i_limit-1;
    let mut spaces2 = 2;

    for i in 1..=i_limit {
        if i <= n {
            for j in 1..=i {print!("*");}
            for _ in 0..spaces {print!(" ");}
            for j in 1..=i {print!("*");}

            spaces -= 2;
        } else {
            for j in 1..=t {print!("*");}
            for _ in 0..spaces2 {print!(" ");}
            for j in 1..=t {print!("*");}

            t -= 1;
            spaces2 += 2;
        }
        println!();
    }
}

// better one
// fn pattern20(n: i32) {
//     let i_limit = n * 2 - 1;

//     for i in 1..=i_limit {
//         let num_stars = if i <= n { i } else { 2 * n - i };
//         let spaces = 2 * (n - num_stars);

//         for _ in 1..=num_stars { print!("*"); }
//         for _ in 0..spaces { print!(" "); }
//         for _ in 1..=num_stars { print!("*"); }
//         println!();
//     }
// }