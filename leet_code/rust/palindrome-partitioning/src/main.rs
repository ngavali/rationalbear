//https://leetcode.com/problems/palindrome-partitioning/

struct Solution;

use std::ops::Range;

impl Solution {
    fn is_palindrome(s: &[char], mut i: usize, mut j: usize) -> bool {
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }

    fn generate_palindromic_partitions_iterative(
        palindromic_partitions: &mut Vec<Vec<String>>,
        s: &[char],
        curr_list: &mut Vec<String>,
    ) {
        let mut stack: Vec<(usize, Range<usize>)> = Vec::new();
        stack.push((0, 0..s.len()));
        while let Some(mut curr_ptr) = stack.pop() {
            let start = curr_ptr.0;
            match curr_ptr.1.next() {
                Some(k) => {
                    if Self::is_palindrome(s, start, k) {
                        curr_list.push(String::from_iter(&s[start..(k + 1)]));
                        stack.push(curr_ptr);
                        stack.push((k + 1, k+1..s.len()));
                    } else {
                        stack.push(curr_ptr);
                    }
                }
                None => {
                    if start >= s.len() {
                        palindromic_partitions.push(curr_list.clone());
                    }
                    curr_list.pop();
                }
            };
        }
    }

    fn generate_palindromic_partitions(
        palindromic_partitions: &mut Vec<Vec<String>>,
        start: usize,
        s: &[char],
        curr_list: &mut Vec<String>,
    ) {
        if start >= s.len() {
            palindromic_partitions.push(curr_list.clone());
            return;
        }
        for k in start..s.len() {
            if Self::is_palindrome(s, start, k) {
                curr_list.push(String::from_iter(&s[start..(k + 1)]));
                Self::generate_palindromic_partitions(palindromic_partitions, k + 1, s, curr_list);
                curr_list.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        //Generate palindromic_partitions
        let s = s.chars().collect::<Vec<char>>();
        let mut palindromic_partitions: Vec<Vec<String>> = Vec::with_capacity(s.len());
        let mut curr_list: Vec<String> = Vec::new();
        //Self::generate_palindromic_partitions(&mut palindromic_partitions, 0, &s, &mut curr_list);
        Self::generate_palindromic_partitions_iterative(&mut palindromic_partitions, &s, &mut curr_list);
        palindromic_partitions
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, Vec<Vec<String>>)> {
        vec![
            (
                "aab".to_string(),
                vec![
                    vec!["a".to_string(), "a".to_string(), "b".to_string()],
                    vec!["aa".to_string(), "b".to_string()],
                ],
            ),
            (
                "abcaa".to_string(),
                vec![
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "c".to_string(),
                        "a".to_string(),
                        "a".to_string(),
                    ],
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "c".to_string(),
                        "aa".to_string(),
                    ],
                ],
            ),
            ("a".to_string(), vec![vec!["a".to_string()]]),
            (
                "aaab".to_string(),
                vec![
                    vec![
                        "a".to_string(),
                        "a".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    vec!["a".to_string(), "aa".to_string(), "b".to_string()],
                    vec!["aa".to_string(), "a".to_string(), "b".to_string()],
                    vec!["aaa".to_string(), "b".to_string()],
                ],
            ),
            (
                "abbab".to_string(),
                vec![
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "b".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    vec!["a".to_string(), "b".to_string(), "bab".to_string()],
                    vec![
                        "a".to_string(),
                        "bb".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    vec!["abba".to_string(), "b".to_string()],
                ],
            ),
            (
                "abaca".to_string(),
                vec![
                    vec![
                        "a".to_string(),
                        "b".to_string(),
                        "a".to_string(),
                        "c".to_string(),
                        "a".to_string(),
                    ],
                    vec!["a".to_string(), "b".to_string(), "aca".to_string()],
                    vec!["aba".to_string(), "c".to_string(), "a".to_string()],
                ],
            ),
            (
                "aaa".to_string(),
                vec![
                    vec!["a".to_string(), "a".to_string(), "a".to_string()],
                    vec!["a".to_string(), "aa".to_string()],
                    vec!["aa".to_string(), "a".to_string()],
                    vec!["aaa".to_string()],
                ],
            ),
        ]
    }

    #[test]
    fn test_partition() {
        for (s, want) in testcases() {
            assert_eq!(Solution::partition(s), want);
        }
    }
}
