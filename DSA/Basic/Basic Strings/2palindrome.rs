fn main() {
    println!("{}", palindrome_check("hannah"));
}

fn palindrome_check(s: &str) -> bool {
    s == (s.chars().rev().collect::<String>())
}