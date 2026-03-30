fn main() {
    println!("{}", array_sorted_or_not(&[1,2,3,5], 4));
}

fn array_sorted_or_not(a: &[i32], n: usize) -> bool {
    for i in 0..n-1 {
        if a[i+1] < a[i] {
            return false;
        }
    }
    true
}