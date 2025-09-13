//https://leetcode.com/problems/sort-vowels-in-a-string/description/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels: HashMap<u8, bool> = vec![b'a', b'e', b'i', b'o', b'u', b'A', b'E',b'I',b'O',b'U']
            .iter()
            .map(|&v| (v, true))
            .collect();
        fn get_vowel(vowels_in_string: &mut [i32;128]) -> Option<u8> {
            let vowels_list = [b'A',b'E',b'I',b'O',b'U',b'a', b'e', b'i', b'o', b'u'];
            let mut i = 0;
            while i < vowels_list.len() {
                if vowels_in_string[vowels_list[i] as usize] != 0 {
                    vowels_in_string[vowels_list[i] as usize] -=1;
                    return Some(vowels_list[i]);
                }
                i+=1;
            }
            None
        }
        let mut chars: Vec<u8> = s.as_bytes().to_vec();
        let mut vowels_in_string: [i32;128] = [0; 128];
        chars
            .clone()
            .into_iter().enumerate()
            .for_each(|(i, c)| {
                if vowels.get(&c).is_some() {
                    vowels_in_string[c as usize]+=1;
                };
            });

        for i in 0..chars.len() {
            if vowels.get(&chars[i]).is_some() { 
                if let Some(v) = get_vowel(&mut vowels_in_string) {
                    chars[i] = v;
                }
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
