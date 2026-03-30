fn main() {
    let mut arr = [1,2,3,4,5];
    reverse(&mut arr, 5);
    println!("{:?}", arr);
}

fn reverse(arr: &mut [i32], n: usize) {
    arr.reverse();
}

// fn reverse(arr: &mut [i32], _n: usize) {
//     // 1. Collect reversed elements into a new heap-allocated Vector
//     let reversed_vec: Vec<i32> = arr.iter().copied().rev().collect();
    
//     // 2. Copy the elements from the Vector back into the mutable slice
//     arr.copy_from_slice(&reversed_vec);
// }