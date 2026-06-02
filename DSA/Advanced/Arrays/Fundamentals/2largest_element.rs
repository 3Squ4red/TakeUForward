fn main() {
    println!("{}", largest_element(&[2, 3, 4, 5, 3]));
}

fn largest_element(arr: &[i32]) -> i32 {
    // *arr.into_iter().max().unwrap()
    let mut max = arr[0];

    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }

    max
}