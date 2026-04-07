fn main() {
    println!("{:?}", selection_sort(&[3, 2, 3, 4, 5]));
}

fn selection_sort(a: &[i32]) -> Vec<i32> {
    let mut arr = a.to_vec();
    let n = arr.len();
    if n < 2 {return arr;}

    for i in 0..n-1 {
        // let (min_o, min) = arr[i..n].iter().enumerate().min_by_key(|&(_, val)| val).unwrap();
        // let min_i = min_o+i;
        let min_i = get_min(&arr[i..n], i);

        if i != min_i {arr.swap(i, min_i)}
    }

    arr
}

fn get_min(a: &[i32], o: usize) -> usize {
    let mut min_i = 0;

    for i in 1..a.len() {
        if a[i] <= a[min_i] {
            min_i = i;
        }
    }

    min_i+o
}