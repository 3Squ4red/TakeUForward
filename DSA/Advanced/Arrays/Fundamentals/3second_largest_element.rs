fn main() {
    println!("{}", second_largest_element(&[8, 8, 7, 6, 5]));
}

fn second_largest_element(arr: &[i32]) -> i32 {
    if arr.len() < 2 {return -1;}

    let mut max = i32::MIN;
    let mut s_max = i32::MIN;

    for i in 0..arr.len() {
        if arr[i] > max {
            s_max = max;
            max = arr[i];
        } else if arr[i] > s_max && arr[i] != max {
            s_max = arr[i];
        }
    }

    if max == s_max || s_max == i32::MIN {return  -1;}
    
    s_max
}