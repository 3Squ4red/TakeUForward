fn main() {
    println!("{:?}", bubble_sort(&[13, 46, 24, 52, 20, 9]));
}

fn bubble_sort(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    if n < 2 {return a.to_vec();}
    let mut arr = a.to_vec();

    for i in 0..n-1 {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1)
            }
        }
    }

    arr
}