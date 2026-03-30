fn main() {
    println!("{}", sum(&[13,311,45,11,78], 5))
}

fn sum(arr: &[i32], _l: usize) -> i32 {
    arr.iter().sum()
}
