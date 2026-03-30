fn main() {
    println!("{}", longest_common_prefix(&[]));
}

fn longest_common_prefix(v: &[&str]) -> String {
    if v.is_empty() {return "".to_string();}

    let mut res = String::new();

    let mut min = v[0].len();

    for s in v {
        if s.len() < min {
            min = s.len();
        }
    }

    for i in 0..min {
        let mut b = v[0].as_bytes();
        let c = b[i];

        for s in &v[1..] {
            b = s.as_bytes();
            if b[i] != c {
                return res;
            }
        }
        res.push(b[i] as char);
    }

    res
}