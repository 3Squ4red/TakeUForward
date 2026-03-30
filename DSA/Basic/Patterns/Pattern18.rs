fn main() {
    pattern18(26)
}

fn pattern18(n: u8) {
    let end_ch = 64 + n;

    for i in 0..n {
        for j in end_ch-i..=end_ch {
            print!("{} ", j as char);
        }
        println!()
    }
}