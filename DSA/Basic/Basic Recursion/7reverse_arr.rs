fn main() {
    println!("{:?}", reverse_array(&mut [1, 2, 3, 4, 5]));
}

fn reverse_array(a: &mut [i32]) -> &mut [i32] {
    let n = a.len();
    if n < 2 {
        a
    } else {
        helper(a, 0, n/2)
    }
}

fn helper(a: &mut [i32], i: usize, mid: usize) -> &mut [i32] {
    if i != mid {
        a.swap(i, a.len()-1-i);
        helper(a, i+1, mid)
    } else {
        a
    }
}


// fn reverse_array(a: &[i32]) -> Vec<i32> {
//     let n = a.len();
//     if n < 2 {
//         a.to_vec()
//     } else {
//         helper(a, &mut vec![0; n], 0)
//     }
// }

// fn helper(a: &[i32], na: &mut[i32], i: usize) -> Vec<i32> {
//     if i < a.len() {
//         na[i] = a[a.len()-1-i];
//         helper(a, na, i+1)
//     }
//     else {na.to_vec()}
// }