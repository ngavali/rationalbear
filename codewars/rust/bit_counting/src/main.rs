//https://www.codewars.com/kata/526571aae218b8ee490006f4/rust
fn count_bits(n: i64) -> u32 {
    /*
    // code here
    let mut bits = 0;
    while n > 0 {
        if n & 1 == 1 {
            bits += 1;
        }
        n >>= 1;
    }
    bits
    */
    n.count_ones()
}

fn main() {
    println!("1's = {:}", count_bits(1234));
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}
