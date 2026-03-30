use std::collections::HashMap;

fn main() {
    println!("{}", sum_highest_and_lowest_frequency(&[10, 9, 7, 7, 8, 8, 8]));
}

fn sum_highest_and_lowest_frequency(a: &[i32]) -> u32 {
    let mut h = HashMap::new();

    for &e in a {
        *h.entry(e).or_insert(0) += 1;
    }

    if h.is_empty() {return 0;}

    let mut low = *h.values().next().unwrap();
    let mut high = low;

    for &i in h.values() {
        if i > high {high = i;}
        if i < low {low = i;}
    }

    high + low
}