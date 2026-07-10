fn main() {
    println!("{}", find_max_consecutive_ones(&[1, 1, 1, 1, 0, 1]));
}

fn find_max_consecutive_ones(arr: &[i32]) -> i32 {
    let mut c = 0;
    let mut r = 0;

    for i in arr {
        if *i == 0 {
            r = if c > r {c} else {r};
            c = 0;
        }
        else {c+=1;}
    }
    
    if c > r {c} else {r}
}