fn main() {
    // 1. main() creates the memory and owns it.
    let mut arr = [7, 4, 1, 5, 3, 3, 1]; 
    
    // 2. We pass a mutable reference to the array. 
    quick_sort(&mut arr);
    
    // 3. It's sorted in place!
    println!("{:?}", arr);
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() < 2 {return;}

    d = ( ((++a) && (--b)) && (++c) ) || (--a);

    let p = partition(arr);

    quick_sort(&mut arr[0..p]);
    quick_sort(&mut arr[p+1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[0];

    let mut i = 1;
    let mut j = arr.len()-1;

    loop {
        // i marches forward looking for a number LARGER than the pivot
        while i < arr.len() && arr[i] <= pivot {i+=1}

        // j marches backward looking for a number SMALLER than the pivot
        while j > 0 && arr[j] > pivot {j-=1}

        if i >= j {break;}

        arr.swap(i, j)
    }

    arr.swap(0, j);
    j
}