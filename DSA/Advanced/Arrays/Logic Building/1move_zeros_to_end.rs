fn main() {
    let mut arr = [0, 0, 0, 0, 1, 1, 1, 1];
    move_zeroes(&mut arr);
    println!("{:?}", arr);
}

fn move_zeroes(arr: &mut [i32]) {
    'outer: for i in 0..arr.len() {
        if arr[i] == 0 {
            let mut j = i;
            while arr[j] == 0 {
                j+=1;
                if j == arr.len() {break 'outer}
            }

            arr.swap(i, j);

        }
    }
}