use std::collections::HashMap;

fn main() {
    println!("{}", isomorphic_string( "apple" ,"bbnbm"));
}

// fn isomorphic_string(s: &str, t: &str) -> bool {
//     if s.len() != t.len() {return false;}

//     let mut placeholder1 = String::new();
//     let mut placeholder2 = String::new();

//     let mut iter1 = 0..s.len();
//     let mut iter2 = 0..s.len();

//     let mut hm1: HashMap<char, String> = HashMap::new();
//     let mut hm2: HashMap<char, String> = HashMap::new();

//     for c in s.chars() {
//         let val = hm1.get(&c);

//         if val.is_some() {
//             placeholder1.push_str(val.unwrap());
//         } else {
//             let id = iter1.next().unwrap().to_string();
//             hm1.insert(c, id.clone());
//             placeholder1.push_str(&id);
//         }
//     }

//     // println!("{:?}", hm1);
//     // println!("{}", placeholder1);

//     for c in t.chars() {
//         let val = hm2.get(&c);

//         if val.is_some() {
//             placeholder2.push_str(val.unwrap());
//         } else {
//             let id = iter2.next().unwrap().to_string();
//             hm2.insert(c, id.clone());
//             placeholder2.push_str(&id);
//         }
//     }

//     // println!("{:?}", hm2);
//     // println!("{}", placeholder2);

//     placeholder1 == placeholder2
// }


// AI
pub fn isomorphic_string(string_a: &str, string_b: &str) -> bool {
    if string_a.len() != string_b.len() {
        return false;
    }

    // Arrays to track the (last seen index) for each ASCII character.
    // In Rust, we can initialize a 256-element array with -1s like this:
    let mut last_seen_position_in_a = [-1; 256];
    let mut last_seen_position_in_b = [-1; 256];

    let bytes_a = string_a.as_bytes();
    let bytes_b = string_b.as_bytes();

    for i in 0..(string_a.len() as i32) {
        // We cast to usize because Rust requires array indices to be of type usize
        let char_from_a = bytes_a[i as usize ] as usize;
        let char_from_b = bytes_b[i as usize] as usize;

        // If their historical positions don't match, the structural pattern is broken
        if last_seen_position_in_a[char_from_a] != last_seen_position_in_b[char_from_b] {
            return false;
        }

        // Record the current occurrence. 
        last_seen_position_in_a[char_from_a] = i;
        last_seen_position_in_b[char_from_b] = i;
    }

    true
}

/*
public static boolean isomorphicStringOptimized(String s, String t) {
    if (s.length() != t.length()) return false;

    // Standard ASCII size map
    int[] mapS = new int[256];
    int[] mapT = new int[256];

    for (int i = 0; i < s.length(); i++) {
        char c1 = s.charAt(i);
        char c2 = t.charAt(i);

        // If the last seen positions of the characters don't match, they aren't isomorphic
        if (mapS[c1] != mapT[c2]) {
            return false;
        }

        // Update the maps with the current index + 1 
        // (+1 ensures we don't confuse index 0 with the default array value of 0)
        mapS[c1] = i + 1;
        mapT[c2] = i + 1;
    }

    return true;
}
*/