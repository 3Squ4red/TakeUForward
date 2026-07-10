fn main() {
    let mut arr = [3, 4, 1, 5, 3, -5];

    rotate_array(&mut arr, 8);

    println!("{:?}", arr);
}

fn rotate_array(arr: &mut [i32], mut k: usize) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    
    k %= n;
    
    arr[0..k].reverse();
    arr[k..].reverse();
    arr[..].reverse();
}

fn rotate_array_n_space(arr: &mut [i32], mut k: i32) {
    let l = arr.len() as i32;
    if l < 2 {
        return;
    }

    let mut a2 = vec![0; arr.len()];
    let j = k % l;
    for i in 0..l {
        let m = (l - j + i) % l;
        a2[m as usize] = arr[i as usize];
    }
    arr.copy_from_slice(&a2);
}

fn rotate_array_recur(arr: &mut [i32], mut k: i32) {
    if arr.len() < 2 {
        return;
    }

    if k > 0 {
        let f = arr[0];
        for i in 1..arr.len() {
            arr[i - 1] = arr[i];
        }
        arr[arr.len() - 1] = f;
    }
    k -= 1;
    if k > 0 {
        rotate_array_recur(arr, k);
    }
}
