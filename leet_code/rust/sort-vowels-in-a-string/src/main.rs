//https://leetcode.com/problems/sort-vowels-in-a-string/description/

struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        fn is_vowel(v: u8) -> bool {
            match v.to_ascii_lowercase() {
                b'a'|b'e'|b'i'|b'o'|b'u' => true,
                _ => false
            }
        }
        let vowels_list = [b'A',b'E',b'I',b'O',b'U',b'a', b'e', b'i', b'o', b'u'];
        let mut chars: Vec<u8> = s.as_bytes().to_vec();
        let mut vowels_in_string: [i32;128] = [0; 128];
        chars
            .iter()
            .for_each(|&c| {
                if is_vowel(c) {
                    vowels_in_string[c as usize]+=1;
                };
            });
        let mut curr_pos = 0;
        for i in 0..chars.len() {
            if is_vowel(chars[i]) { 
                while vowels_in_string[vowels_list[curr_pos] as usize] == 0 {
                    curr_pos+=1;
                }
                vowels_in_string[vowels_list[curr_pos] as usize] -=1;
                chars[i] = vowels_list[curr_pos];
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
