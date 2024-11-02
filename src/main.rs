use std::env; // import the std::env module
use std::str::FromStr; // import FromStr trait from standard library ... to use from_str

fn main() {
    let mut numbers = Vec::new(); // declare mutable vector of numbers

    // iterate over results of env::args call, skipping first item (program name)
    for arg in env::args().skip(1) {
        /* numbers.push(u64::from_str(&arg).expect("error parsing argument")); */

        // invoke u64s from_str class/static method ... which returns a Result
        let r = u64::from_str(&arg);

        numbers.push(r.expect("error parsing argument"));

        // Rust does not have exceptions: all errors are handled using either Result or panic
    }

    if numbers.len() == 0 {
        // eprintln! -> write to std err
        // “raw string” syntax: regex: r[#]+"
        eprintln!(
            r#"
        Usage: gcd NUMBERS
        $ gcd 1
        # gcd 3 6 12
        # gcd 4 8 24 ... 2
        "#
        ); // num of # matches r#+ from above
        std::process::exit(1);
    }

    let mut d = numbers[0];

    // &numbers -> get a reference to (but not ownership of) the elements of
    for m in &numbers[1..] {
        // [1..] -> second element onwards
        // *m -> dereference the reference to read the value
        d = greatest_cmn_divisor(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// mut -> declare mutable variable
// u64 -> unsigned int
fn greatest_cmn_divisor(mut n: u64, mut m: u64) -> u64 {
    // `!` -> macro, not function
    // use debug_assert! instead to skip during compilation
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m; // declare local variable
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // last expression automatically returned.
      //Use explicit `return` statement only when early return is needed
}

#[test] // add `test` attribute -> skipped in std compile, but compiled for `cargo test`
fn test_gcd() {
    assert_eq!(greatest_cmn_divisor(14, 15), 1);
    assert_eq!(greatest_cmn_divisor(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
