/*
 * https://leetcode.com/problems/reverse-bits/
 *
 */
struct Solution;
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut y = 0;
        for bits in 0..32 {
            y = (y << 1) | (x & 0x01);
            x >>= 1;
        }
        y
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_reverse_bits() {
        let test_cases = vec![
            (
                0b10100101000001111010011100,
                0b111001011110000010100101000000,
            ),
            (
                0b11111111111111111111111111111101,
                0b10111111111111111111111111111111,
            ),
        ];
        for test_case in test_cases {
            assert_eq!(test_case.1, Solution::reverse_bits(test_case.0));
        }
    }
}
