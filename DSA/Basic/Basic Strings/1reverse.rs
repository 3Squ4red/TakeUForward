fn main() {
    let mut ls = ["h", "e" ,"l" ,"l" ,"o"];
    reverse_string(&mut ls);
    println!("{:?}", ls);
}

fn reverse_string(ls: &mut [&str]) {
    ls.reverse()
}

// fn reverse_string(ls: &mut [&str]) {
//     let v: Vec<&str> = ls.iter().copied().rev().collect();
//     // *ls = v.as_slice(); // this is stupid, when will u learn?
//     ls.copy_from_slice(&v);
// }