fn main() {
    println!("{}", is_sorted(&[1,9,6,8,5,4,0]));
}

fn is_sorted(a: &[i32]) -> bool {
    if a.len() < 2 {return true;}
    helper(a, 0)
}

fn helper(a: &[i32], i: usize) -> bool {
    let curr = a[i] <= a[i+1];
    
    if curr {
        if i == a.len()-2 {curr}
        else {curr && helper(a, i+1)}
    }
    else {false}
}

