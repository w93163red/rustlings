// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)


#[allow(clippy::for_loop_over_option)]
fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);
}
