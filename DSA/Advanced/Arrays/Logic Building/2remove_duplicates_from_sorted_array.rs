fn main() {
    let mut arr = [0, 0, 3, 3, 5, 6];
    println!("{}", remove_duplicates(&mut arr));
    println!("{:?}", arr);
}

fn remove_duplicates(arr: &mut [i32]) -> usize {
    // The Sniper Guard
    if arr.is_empty() {
        return 0;
    }

    let mut i = 0;
    
    for j in 1..arr.len() {
        if arr[j] != arr[i] {
            i += 1; // Increment the slow pointer first
            arr[i] = arr[j]; // Drop the new unique value into place
        }
    }
    
    i + 1
}
