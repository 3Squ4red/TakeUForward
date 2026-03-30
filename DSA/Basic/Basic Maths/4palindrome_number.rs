fn main() {
    print!("{}", is_palindrome(12234221));
}

// fn is_palindrome(n: i32) -> bool {
//     let s = n.to_string();

//     s == s.chars().rev().collect::<String>()
// }

fn is_palindrome(n: i32) -> bool {
    let mut reversed = 0;
    let mut temp = n;

    while temp != 0 {
        reversed = (reversed * 10) + temp % 10;
        temp /= 10;
    }

    n == reversed
}

