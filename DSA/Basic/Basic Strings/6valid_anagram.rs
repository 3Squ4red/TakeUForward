use std::collections::HashMap;

fn main() {
    println!("{}", anagram_strings("anagram", "nagaram"));
}

// AI
fn anagram_strings(s: &str, t: &str) -> bool {
    if s.len() != t.len() {return false;}

    let mut counts = [0; 256];

    for c in s.bytes() {
        counts[c as usize] += 1;
    }

    for c in t.bytes() {
        counts[c as usize] -= 1;
    }

    counts == [0; 256]
}

// fn anagram_strings(s: &str, t: &str) -> bool {
//     if s.len() != t.len() {return false;}

//     let mut shm = HashMap::new();
//     let mut thm = HashMap::new();

//     for c in s.bytes() {
//         *shm.entry(c).or_insert(0) += 1;
//     }

//     for c in t.bytes() {
//         *thm.entry(c).or_insert(0) += 1;
//     }

//     shm == thm
// }