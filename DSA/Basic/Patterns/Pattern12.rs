fn main() {
    pattern12(4)
}

fn pattern12(n: i32) {
    let j_limit = n*2;

    for i in 1..=n {
        // for j in 1..=j_limit {
        //     if j >= (i+1) && j <= j_limit-i {
        //         print!(" ");
        //     } else {
        //         print!("{}", j);                
        //     }
        // }
        for j in 1..=i {print!("{}", j);}
        for _ in i+1..=j_limit-i {print!(" ");}
        for j in (1..=i).rev() {print!("{}", j);}
        println!()
    }
}