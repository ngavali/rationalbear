fn gcd( mut x: u64, mut y : u64) -> u64 {
    assert!( x !=0 && y!= 0 );
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

#[test]
fn test_gcd() {
    assert_eq!( gcd(14, 15), 1 );
    assert_eq!( gcd(2*3*5*11*17, 3*7*11*13*19), 3*11 );
}
