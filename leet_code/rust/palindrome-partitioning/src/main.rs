//https://leetcode.com/problems/palindrome-partitioning/

struct Solution;

impl Solution {
    fn is_palindrome(s: &[u8], start: usize, end: usize, dp: &mut Vec<Vec<bool>>) -> bool {
        if end - start >= 2  {
            return dp[start+1][end-1] && s[start] == s[end];
        }
        dp[start][end]
    }
    fn generate_palindromic_partitions(
        palindromic_partitions: &mut Vec<Vec<String>>,
        start: usize,
        s: &[u8],
        curr_list: &mut Vec<String>,
        dp: &mut Vec<Vec<bool>>
    ) {
        if start >= s.len() {
            palindromic_partitions.push(curr_list.clone());
            return;
        }
        for k in start..s.len() {
            println!("{} {}", start, k);
            if Self::is_palindrome(&s, start, k, dp) {
                dp[start][k] = true;
                curr_list.push(String::from_utf8(s[start..(k+1)].to_vec()).unwrap());
                Self::generate_palindromic_partitions(
                    palindromic_partitions,
                    k+1,
                    s,
                    curr_list,
                    dp
                );
                curr_list.pop();
            }
            /*
            if start + k <= s.len() && Self::is_palindrome(&s, start, start + k - 1, dp) {
                dp[start][start + k -1] = true;
                curr_list.push(String::from_utf8(s[start..start + k].to_vec()).unwrap());
                Self::generate_palindromic_partitions(
                    palindromic_partitions,
                    start + k,
                    s,
                    curr_list,
                    dp
                );
                curr_list.pop();
            }*/
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        //Generate palindromic_partitions
        let s = s.as_bytes();
        let mut palindromic_partitions: Vec<Vec<String>> = Vec::with_capacity(s.len());
        let mut curr_list: Vec<String> = Vec::new();
        let mut dp = vec![vec![false;s.len()];s.len()];

        for i in 0..s.len() {
            dp[i][i] = true;
        }
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = true;
            }
        }
        Self::generate_palindromic_partitions(
            &mut palindromic_partitions,
            0,
            s,
            &mut curr_list,
            &mut dp
        );
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
