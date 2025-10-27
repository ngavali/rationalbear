//https://leetcode.com/problems/longest-common-prefix/
struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        let min_str_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let fs = strs[0].clone();
        for i in 0..min_str_len {
            for s in strs[1..].iter() {
                if s[i] != fs[i] {
                    return String::from_iter(&fs[0..i]);
                }
            }
        }
        String::from_iter(&fs[0..min_str_len])
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<String>, String)> {
        vec![
            (
                vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                "fl".to_string(),
            ),
            (
                vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                "".to_string(),
            ),
            (
                vec![
                    "reflower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                "".to_string(),
            ),
            (
                vec![
                    "aaaa".to_string(),
                    "aaaaaaa".to_string(),
                    "a".to_string(),
                    "aaaaaaa".to_string(),
                ],
                "a".to_string(),
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_longest_common_prefix() {
        for (strs, want) in testcases() {
            assert_eq!(Solution::longest_common_prefix(strs), want);
        }
    }
}
