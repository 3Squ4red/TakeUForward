/*
Patterns

1. Figure out the no. of lines to be printed & write the outer loop accordingly.
2. Figure out what is happening at every line & try to connect with the outer loop if possible. Write the inner loop based on this.
3. Execute print when needed.
4. Observe symmetry. (optional)
*/

fn main() {
    pattern1(4);
}

fn pattern1(n: i32) {
    for _ in 0..n {
        for _ in 0..n {
            print!("*");
        }
        println!();
    }
}