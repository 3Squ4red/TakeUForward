// this one took a lot of time to figure out
// was confused about how to handle `d`

fn main() {
    pattern17(5)
}

fn pattern17(n: u8) {
    let mut s = 65;
    let mut c = 0;

    for i in (0..n).rev() {

        for _ in 1..=i {
            print!(" ");
        }

        let ch_limit = 65 + c;
        let mut d = 1;
        for j in 65_u8..=s {
            if j <= ch_limit {
                print!("{}", j as char);
            } else {
                print!("{}", (ch_limit - d) as char);
                d+=1;
            }
        }

        s += 2;
        c += 1;

        println!();
    }
}
