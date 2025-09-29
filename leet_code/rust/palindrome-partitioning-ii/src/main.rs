//https://leetcode.com/problems/palindrome-partitioning-ii/

struct Solution;

use std::ops::Range;

impl Solution {
    fn is_palindrome(s: &[char], mut i: usize, mut j: usize, dp: &mut Vec<Vec<i32>>) -> bool {
        let idx = (i,j);
        if dp[idx.0][idx.1] == -1 {
            dp[idx.0][idx.1] = 1;
            while i < j {
                if s[i] != s[j] {
                    dp[idx.0][idx.1] = 0;
                    break;
                }
                i += 1;
                j -= 1;
            }
        }
        return dp[idx.0][idx.1] == 1;
    }

    fn generate_palindromic_partitions_iterative(
        partitions: &mut usize,
        s: &[char],
        curr_list: &mut Vec<String>,
        dp: &mut Vec<Vec<i32>>
    ) {
        let mut stack: Vec<(usize, Range<usize>)> = Vec::new();
        stack.push((0, 0..s.len()));
        while let Some(mut curr_ptr) = stack.pop() {
            let start = curr_ptr.0;
            match curr_ptr.1.next() {
                Some(k) => {
                    if Self::is_palindrome(s, start, k, dp) {
                        dp[start][k] = 1;
                        curr_list.push(String::from_iter(&s[start..(k + 1)]));
                        stack.push(curr_ptr);
                        stack.push((k + 1, k + 1..s.len()));
                    } else {
                        stack.push(curr_ptr);
                    }
                }
                None => {
                    if start >= s.len() {
                        *partitions = curr_list.len().min(*partitions);
                    }
                    curr_list.pop();
                }
            };
        }
    }

    fn generate_palindromic_partitions(
        partitions: &mut i32,
        start: usize,
        s: &[char],
        curr_list: &mut Vec<String>,
        memo: &mut Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>
    ) -> i32 {
        //If the remaining string is palidrome then we return immediately
        if start >= s.len() || Self::is_palindrome( &s, start, s.len()-1, dp ) {
            return 0;
        }
        if memo[start][s.len()-1] != -1 {
            return memo[start][s.len()-1];
        }
        let mut mincut = i32::MAX;
        for k in start..s.len() {
            if s[start] == s[k] && ( k-start < 2 || Self::is_palindrome( &s, start+1, k-1, dp )) {
                dp[start][k] = 1;
                curr_list.push(String::from_iter(&s[start..(k + 1)]));
                mincut = mincut.min( 1 + Self::generate_palindromic_partitions(
                    partitions,
                    k + 1,
                    s,
                    curr_list,
                    memo,
                    dp
                ));
                curr_list.pop();
            } else {
                dp[start][k] = 0;
            }
        }
        memo[start][s.len()-1] = mincut;
        mincut
    }

    pub fn min_cut(s: String) -> i32 {
        //Generate palindromic_partitions
        let s = s.chars().collect::<Vec<char>>();
        let mut mincut = i32::MAX;
        let mut curr_list: Vec<String> = Vec::new();
        let mut memo = vec![vec![-1; s.len()]; s.len()];
        let mut dp = vec![vec![-1; s.len()]; s.len()];
        //Self::generate_palindromic_partitions_iterative(&mut mincut, &s, &mut curr_list);
        for i in 0..s.len() {
            dp[i][i] = 1;
            if i < s.len() - 1 {
                if s[i] == s[i+1] {
                    dp[i][i+1] = 1;
                }
            }
        }
        match Self::generate_palindromic_partitions(
            &mut mincut,
            0,
            &s,
            &mut curr_list,
            &mut memo,
            &mut dp
        ) {
            i32::MAX => 0,
            num => {
                num
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, i32)> {
        vec![
            ("aab".to_string(), 1),
            ("abcaa".to_string(), 3),
            ("a".to_string(), 0),
            ("aaab".to_string(), 1),
            ("abbab".to_string(), 1),
            ("abaca".to_string(), 2),
            ("aaa".to_string(), 0),
            (
                "ababababababababababababcbabababababababababababa".to_string(),
                0,
            ),
            ("ccaacabacb".to_string(), 3),
            (
                "apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp".to_string(),
                452
            ),
            (
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
                1
            ),
        ]
    }

    #[test]
    fn test_partition() {
        for (s, want) in testcases() {
            assert_eq!(Solution::min_cut(s), want);
        }
    }
}
