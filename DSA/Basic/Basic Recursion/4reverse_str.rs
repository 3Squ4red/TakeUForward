fn main() {
    println!("{:?}", reverse_string(&mut "hello".chars().collect::<Vec<char>>()));
}

fn reverse_string(a: &mut Vec<char>) -> &mut Vec<char> {
    let n = a.len();
    if n < 2 {
        a
    } else {
        helper(a, 0, n/2, n)
    }
}

fn helper(a: &mut Vec<char>, i: usize, mid: usize, n: usize) -> &mut Vec<char> {
    if i != mid {
        a.swap(i, n-1-i);
        helper(a, i+1, mid, n)
    } else {
        a
    }
}