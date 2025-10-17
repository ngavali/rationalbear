//https://leetcode.com/problems/repeated-dna-sequences/

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn encode_char(c: char) -> usize {
        match c {
            'C' => 0b01,
            'G' => 0b10,
            'T' => 0b11,
            _ => 0b00,
        }
    }
    fn generator() -> HashMap<String, usize> {
        let mut str_map = HashMap::new();
        for n1 in vec!['A', 'C', 'G', 'T'] {
            for n2 in vec!['A', 'C', 'G', 'T'] {
                let c1 = Self::encode_char(n1) << 2;
                let c2 = Self::encode_char(n2);
                let com = c1 | c2;
                let v = 1 << com;
                println!(" {com:2} {com:4b} -> value {v:20} {v:32b}");
                str_map.insert(String::from_iter(vec![n1, n2]), v);
            }
        }
        str_map
    }
    fn encode(s: &[char]) -> usize {
        let mut number = 0;
        for c in s {
            number <<= 2;
            match c {
                'C' => number |= 0b01,
                'G' => number |= 0b10,
                'T' => number |= 0b11,
                _ => {}
            };
        }
        println!("{} -> {number:10} -> {number:32b}", String::from_iter(s));
        number
    }

    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let two_str_map = Self::generator();
        println!(" Map of two chars :- {two_str_map:#?}");

        let mut ans = Vec::new();
        let mut str_map = vec![-1; 4194301];

        println!("------ {s} ------- ");

        if s.len() > 10 {
            let s = s.chars().collect::<Vec<char>>();
            let mut encoded_value = Self::encode(&s[0..10]);
            str_map[encoded_value] = 0;

            for i in 10..s.len() {
                encoded_value = (encoded_value<<2 & 0b11111111111111111111 ) | match s[i] {
                    'C' => 0b01,
                    'G' => 0b10,
                    'T' => 0b11,
                    _ => 0b00,
                };
                if str_map[encoded_value] == -1 {
                    str_map[encoded_value] = 0;
                } else if str_map[encoded_value] == 0 {
                    str_map[encoded_value] = 1;
                    let ss = String::from_iter(&s[i-9..=i]);
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
