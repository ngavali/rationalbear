use std::env;
use std::str::FromStr;

fn gcd(mut x: u64, mut y: u64) -> u64 {
    assert!(x != 0 && y != 0);
    while x != 0 {
        if x < y {
            let t = x;
            x = y;
            y = t;
        }
        x = x % y
    }
    y
}

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for number in env::args().skip(1) {
        numbers.push(u64::from_str(&number).expect("Expected a number."));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: program NUMBER1 NUMBER2 ...");
        std::process::exit(1);
    }
    let mut g = numbers[0];
    //for &num in &numbers[1..] {
    for &num in numbers.iter().skip(1) {
        g = gcd(g, num)
    }

    println!("GCD of {:?} is {}", numbers, g);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
