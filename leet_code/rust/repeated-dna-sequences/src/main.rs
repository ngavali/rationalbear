//https://leetcode.com/problems/repeated-dna-sequences/

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn encode(s: &[char]) -> usize {
        let mut number = 0;
        for c in s {
            match c {
                'C' => number |= 0b01,
                'G' => number |= 0b10,
                'T' => number |= 0b11,
                _ => {}
            };
            number <<= 2;
        }
        number
    }
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut str_map = vec![-1; 4194301];
        if s.len() > 10 {
            let s = s.chars().collect::<Vec<char>>();
            for i in 0..s.len() - 9 {
                let encoded_value = Self::encode(&s[i..i+10]);
                if str_map[encoded_value] == -1 {
                    str_map[encoded_value] = 0;
                } else if str_map[encoded_value] == 0{
                    str_map[encoded_value] = 1;
                    let ss = String::from_iter(&s[i..i + 10]);
                println!("String: {ss} -> Encoded value: {encoded_value:10}" );
                    ans.push(ss);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(String, Vec<String>)> {
        vec![
            ("TTTTTTTTTTT".to_string(), vec!["TTTTTTTTTT".to_string()]),
            (
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string(),
                vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()],
            ),
            ("AAAAAAAAAAAAA".to_string(), vec!["AAAAAAAAAA".to_string()]),
            ("A".to_string(), vec![]),
            ("AAAAAAAAAAA".to_string(), vec!["AAAAAAAAAA".to_string()]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_find_repeated_dna_sequences() {
        for (s, want) in testcases() {
            assert_eq!(Solution::find_repeated_dna_sequences(s), want);
        }
    }
}
