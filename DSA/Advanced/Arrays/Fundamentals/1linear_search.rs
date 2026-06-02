fn main() {
    println!("{}", linear_search(&[2, 3, 4, 5, 3], 4));
}

fn linear_search(arr: &[i32], target: i32) -> i32 {
    for i in 0..arr.len() {
        if arr[i] == target {
            return i as i32;
        }
    }

    -1
}
