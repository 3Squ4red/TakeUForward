fn main() {
    println!("{:?}", insertion_sort(&[7, 4, 1, 5, 3]));
}

fn insertion_sort(a: &[i32]) -> Vec<i32> {
    let mut arr = a.to_vec();
    let n = arr.len();
    if n < 2 {return arr;}

    for i in 1..n {
        for j in (1..=i).rev() {
            if arr[j] < arr[j-1] {
                arr.swap(j, j-1);
            } 
            // AI optimization
            else {break;}
        }
    }

    arr
}