fn main() {
    pattern11(5);
}

pub fn pattern11(n: i32) {
    let mut v;

    for i in 1..=n {
        if i % 2 == 1 {
            v = 1;
        } else {
            v = 0;
        }

        for _ in 1..=i {
            print!("{} ", v);
            toggle(&mut v);
        }

        println!();
    }
}

pub fn toggle(n: &mut i32) {
    if *n == 0 {
        *n = 1;
    } else {
        *n = 0;
    }
}
