use std::collections::HashMap;
use std::cmp::Reverse;

fn main() {
    println!("{:?}", frequency_sort("bbccddaaa"));
}

fn frequency_sort(s: &str) -> Vec<char> {
    let mut hm = HashMap::new();

    for c in s.chars() {
        *hm.entry(c).or_insert(0) += 1;
    }

    let mut hmv: Vec<(char, u32)> = hm.into_iter().collect();
    hmv.sort_by_key(|&(c, f)| (Reverse(f), c as u8));

    hmv.into_iter().map(|(c, _)| c).collect()
}


// AI. SC: O(1) vs O(k (# of unique chars)) in mine
// fn frequency_sort_optimized(s: &str) -> Vec<char> {
//     // 1. Track frequencies in a fixed 256-element array
//     let mut counts = [0; 256];
//     for b in s.bytes() {
//         counts[b as usize] += 1;
//     }

//     // 2. Collect only the unique characters that actually appeared
//     let mut unique_chars: Vec<char> = (0..256)
//         .filter(|&i| counts[i] > 0)
//         .map(|i| i as u8 as char)
//         .collect();

//     // 3. Sort by frequency (descending), then by ASCII value (ascending)
//     unique_chars.sort_unstable_by_key(|&c| {
//         (Reverse(counts[c as usize]), c as u8)
//     });

//     unique_chars
// }
