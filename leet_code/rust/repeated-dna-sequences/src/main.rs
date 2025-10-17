//https://leetcode.com/problems/repeated-dna-sequences/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        if s.len() > 10 {
            let mut seq_freq = HashMap::<String, bool>::with_capacity(s.len());
            let s: Vec<char> = s.chars().collect();
            for i in 0..s.len() - 9 {
                let s = String::from_iter(&s[i..i + 10]);
                if let Some(freq) = seq_freq.get_mut(&s) {
                    if !*freq {
                        ans.push(s);
                        *freq = true;
                    }
                } else {
                    seq_freq.insert(s, false);
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
            assert_eq!( Solution::find_repeated_dna_sequences(s), want );
        }
    }
}
