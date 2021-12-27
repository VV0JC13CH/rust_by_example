//
// Based on an example from “Programming Rust”, by Jim Blandy, Jason Orendorff, and Leonora Tindall (MIT License).
// https://github.com/ProgrammingRust/examples/tree/master/gcd
//

use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;

        }
        m = m % n;
    }
    n
}

#[test] // attribute
fn test_gcd(){
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(10,30), 10);
    assert_eq!(gcd(6,18), 6);

}


fn main() {
    // Empty array type. Elements are going to be .pushed below
    let mut numbers = Vec::new();                       

    // The first element is traditionally the path of the executable
    for arg in std::env::args().skip(1) {               
                // from_str returns [Ok] or [Err]. .expect is required to get contained value.
                numbers.push(u64::from_str(&arg).expect("Invalid argument. Parsing error"));
    }

    if numbers.len() <= 1 {
        writeln!(std::io::stderr(), "Pass at least two numbers as arguments").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // & - points to reference, * - points to value, dereference
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("Greatest common divisor of {:?} is {}", numbers, d)
}
