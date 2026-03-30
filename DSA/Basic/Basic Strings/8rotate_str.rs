fn main() {
    println!("{}", rotate_string("abcde", "eabcd"));
}

// fn rotate_string(s: &str, goal: &str) -> bool {
//     let n = s.len();
//     if n != goal.len() {return false;}

//     let bytes = s.as_bytes(); 
//     let mut c = s.to_string();
//     c.push_str(s);

//     for i in 0..n {
//         if &c[i..n+i] == goal {return true;}
//         c.replace_range(n+i..n+i+1, &(bytes[i] as char).to_string());
//     }

//     false
// }

// AI 🤖
fn rotate_string(s: &str, goal: &str) -> bool {
    if s.len() != goal.len() {return false;}
    format!("{s}{s}").contains(goal)
}
