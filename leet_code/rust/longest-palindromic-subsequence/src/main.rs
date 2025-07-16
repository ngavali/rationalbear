/* Medium
 * https://leetcode.com/problems/longest-palindromic-subsequence/
 */

//Longest Common Subsequence

#![feature(test)]

struct Solution;

impl Solution {
    fn find_palindrome(i: usize, j: usize, s: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[i][j] != 0 {
            return memo[i][j];
        }
        if i > j {
            return 0;
        }
        if i == j {
            return 1;
        }
        if s[i] == s[j] {
            memo[i][j] = Solution::find_palindrome(i + 1, j - 1, s, memo) + 2;
        } else {
            memo[i][j] = Solution::find_palindrome(i + 1, j, s, memo)
                .max(Solution::find_palindrome(i, j - 1, s, memo));
        }

        memo[i][j]
    }

    pub fn longest_palindrome_subseq_td(s: String) -> i32 {
        let mut memo = vec![vec![0; s.len()]; s.len()];
        let s = s.as_bytes();

        Solution::find_palindrome(0, s.len() - 1, &s, &mut memo)
    }

    pub fn longest_palindrome_subseq_bu(s: String) -> i32 {
        let mut memo = vec![vec![0; s.len()]; s.len()];
        let s = s.as_bytes();
        for i in { 0..s.len() }.rev() {
            memo[i][i] = 1;
            for j in i + 1..s.len() {
                memo[i][j] = match s[i] == s[j] {
                    true => memo[i + 1][j - 1] + 2,
                    false => memo[i + 1][j].max(memo[i][j - 1]),
                };
            }
        }
        memo[0][s.len() - 1]
    }

    pub fn longest_palindrome_subseq_bu_space_optimized(s: String) -> i32 {
        let mut dp_prev = vec![0; s.len()];
        let mut dp = vec![0; s.len()];
        let s = s.as_bytes();
        for i in { 0..s.len() }.rev() {
            dp[i] = 1;
            for j in i + 1..s.len() {
                dp[j] = match s[i] == s[j] {
                    true => dp_prev[j - 1] + 2,
                    false => dp_prev[j].max(dp[j - 1]),
                };
            }
            dp.iter().enumerate().for_each(|(i, &x)| dp_prev[i] = x);
        }
        dp[s.len() - 1]
    }
}

fn main() {
    println!("Hello, world!");
}

fn testcases() -> Vec<(String, i32)> {
    vec![
        (String::from("bbbab"), 4),
        (String::from("cbbd"), 2),
        (String::from("a"), 1),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_longest_palindrome_subseq_td() {
        for (i, o) in testcases() {
            assert_eq!(o, Solution::longest_palindrome_subseq_td(i))
        }
    }
    #[test]
    fn test_longest_palindrome_subseq_bu() {
        for (i, o) in testcases() {
            assert_eq!(o, Solution::longest_palindrome_subseq_bu(i))
        }
    }
    #[test]
    fn test_longest_palindrome_subseq_bu_space_optimized() {
        for (i, o) in testcases() {
            assert_eq!(o, Solution::longest_palindrome_subseq_bu_space_optimized(i))
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_longest_palindrome_subseq_td(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                assert_eq!(o, Solution::longest_palindrome_subseq_td(i))
            }
        });
    }
    #[bench]
    fn bench_longest_palindrome_subseq_bu(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                assert_eq!(o, Solution::longest_palindrome_subseq_bu(i))
            }
        });
    }
    #[bench]
    fn bench_longest_palindrome_subseq_bu_space_optimized(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                assert_eq!(o, Solution::longest_palindrome_subseq_bu_space_optimized(i))
            }
        });
    }
}
