use std::collections::HashMap;
use std::cmp::Reverse;

fn main() {
    println!("{}", second_most_frequent_element(&[4, 4, 5, 5, 6, 7]));
}

fn second_most_frequent_element(a: &[i32]) -> i32 {
    let mut h = HashMap::new();
    for &e in a {
        *h.entry(e).or_insert(0) += 1;
    }

    // Safety net: If there are fewer than 2 unique items, there is no "second"
    if h.len() < 2 {
        return -1;
    }

    // Dump the HashMap into a Vector so we can sort it
    let mut entries: Vec<(i32, i32)> = h.into_iter().collect();

    // Sort the Vector
    entries.sort_by_key(|&(val, count)| (Reverse(count), val));

    let res = entries[0].1;

    for (val, fre) in entries {
        // return the first val which has different frequency than the first one
        if fre != res {
            return val;
        }
    }

    -1
}

/*
3. The Magic Key: (Reverse(count), val)
This is the core of the logic. You are telling Rust to sort the list based on a brand-new, temporary tuple that you are generating on the fly.

Rust evaluates tuples strictly left-to-right.

The Primary Rule (Reverse(count)): Rust looks at the first item in your generated tuple. Because you want the highest frequencies at the front of the list, but Rust sorts smallest-to-largest by default, you wrap the count in Reverse(). This tricks the sorter. It flips the natural order upside down, forcing the biggest counts to the front of the line.

The Tie-Breaker (val): What happens if two numbers have the exact same frequency? Rust moves to the second item in the tuple to break the tie. Because you just passed val normally (without Reverse), it falls back to its default behavior: sorting the tying values in normal, ascending order (smallest to largest).
*/
