fn main() {
    println!("{}", palindrome_check("aba"));
}

fn palindrome_check(s: &str) -> bool {
    let n = s.len();
    if n < 2 {
        return true;
    } else {
        helper(s.as_bytes(), 0, n / 2, n)
    }
}

fn helper(b: &[u8], i: usize, mid: usize, n: usize) -> bool {
    let curr = b[i] == b[n - 1 - i];
    if i != mid && n > 3 {
        curr && helper(b, i + 1, mid, n)
    } else {
        curr
    }
}

// I miss this one 😢
// fn palindrome_check(s: &str) -> bool {
//     s == s.chars().rev().collect::<String>()
// }

// another crazy sol by me
// fn palindrome_check(s: &str) -> bool {
//     s == reverse_string(&mut s.chars().collect::<Vec<char>>()).into_iter().map(|c: &mut char| *c).collect::<String>()
// }

// fn reverse_string(a: &mut Vec<char>) -> &mut Vec<char> {
//     let n = a.len();
//     if n < 2 {
//         a
//     } else {
//         helper(a, 0, n/2, n)
//     }
// }

// fn helper(a: &mut Vec<char>, i: usize, mid: usize, n: usize) -> &mut Vec<char> {
//     if i != mid {
//         a.swap(i, n-1-i);
//         helper(a, i+1, mid, n)
//     } else {
//         a
//     }
// }
