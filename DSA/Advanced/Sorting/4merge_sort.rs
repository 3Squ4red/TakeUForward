fn main() {
    println!("{:?}", merge_sort(&[7, 4, 1, 5, 3, 3, 1]));
    // println!("{:?}", merge(&[1,1,2,3,4], &[2,4,5,6]));
    // println!("{:?}", merge(&[], &[2,4,5,6]));
}

fn merge_sort(a: &[i32]) -> Vec<i32> {
    let n = a.len();

    if n < 2 {return a.to_vec()}
    let mid = n/2;
    merge(&merge_sort(&a[0..mid]), &merge_sort(&a[mid..]))
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut arr = vec![];
    let mut pa: usize = 0;
    let mut pb: usize = 0;
    
    // following flags not needed
    // let mut pa_exceeded = false;
    // let mut pb_exceeded = false;

    for _ in 0..a.len()+b.len() {
        if pa >= a.len() || pb >= b.len() {
            break;
        }

        if a[pa] <= b[pb] {
            arr.push(a[pa]);
            pa+=1;
        } else {
            arr.push(b[pb]);
            pb+=1;
        }
    }

    arr.extend_from_slice(&b[pb..]);
    arr.extend_from_slice(&a[pa..]);

    arr
}

// AI
// fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
//     let mut arr = Vec::with_capacity(a.len() + b.len()); // Pro-tip: pre-allocate memory!
//     let mut pa = 0;
//     let mut pb = 0;

//     // Only run while BOTH pointers are safely inside their arrays
//     while pa < a.len() && pb < b.len() {
//         if a[pa] <= b[pb] {
//             arr.push(a[pa]);
//             pa += 1;
//         } else {
//             arr.push(b[pb]);
//             pb += 1;
//         }
//     }

//     // When the while loop ends, one of them exceeded their bounds.
//     // We don't even need flags to check which one, we just extend both!
//     // (If pa is at the end, a[pa..] is empty and does nothing. It's totally safe).
//     arr.extend_from_slice(&a[pa..]);
//     arr.extend_from_slice(&b[pb..]);

//     arr
// }
