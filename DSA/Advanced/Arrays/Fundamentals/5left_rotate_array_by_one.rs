fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    rotate_array_by_one(&mut arr);

    println!("{:?}", arr);
}

fn rotate_array_by_one(arr: &mut [i32]) {
    let f = arr[0];
    for i in 1..arr.len() {
        arr[i-1] = arr[i];
    }
    arr[arr.len()-1] = f;
}
