//https://leetcode.com/problems/sort-vowels-in-a-string/description/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels: HashMap<u8, bool> = vec![b'a', b'e', b'i', b'o', b'u', b'A', b'E',b'I',b'O',b'U']
            .iter()
            .map(|&v| (v, true))
            .collect();
        let mut chars: Vec<u8> = s.as_bytes().to_vec();
        let mut vowels_in_string: Vec<u8> = chars
            .clone()
            .into_iter()
            .filter(|&c| vowels.get(&c).is_some() )
            .collect();
        vowels_in_string.sort();
        vowels_in_string.reverse();
        for i in 0..s.len() {
            if vowels.get(&chars[i]).is_some() {
                chars[i] = vowels_in_string.pop().unwrap();
            }
        }
        String::from_utf8(chars).unwrap_or(s)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(String, String)> {
        vec![
            ("lEetcOde".to_string(), "lEOtcede".to_string()),
            ("lYmpH".to_string(), "lYmpH".to_string()),
        ]
    }

    #[test]
    fn test_sort_vowels() {
        for (s, want) in testcases() {
            assert_eq!(Solution::sort_vowels(s), want);
        }
    }
}
