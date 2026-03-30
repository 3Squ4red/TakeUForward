fn main() {
    println!("{:?}", large_odd_num("0214638"));
}

fn large_odd_num(s: &str) -> &str {
    let bytes = s.as_bytes();

    for i in (0..s.len()).rev() {
        if (bytes[i] - b'0') % 2 & 1 == 1 {
            for j in 0..=i {
                if bytes[j] != b'0' {
                    return &s[j..=i];
                }
            }
        }
    }

    ""
}

/*
char vs &str: The .parse() method is defined for strings, not single characters. For chars, `to_digit(<base>)` must be used.
*/