//https://leetcode.com/problems/palindrome-partitioning-ii/

struct Solution;

use std::ops::Range;

impl Solution {
    fn is_palindrome(s: &[char], mut i: usize, mut j: usize, dp: &mut Vec<Vec<i32>>) -> bool {
        let idx = (i, j);
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

    fn generate_palindromic_partitions_top_down(
        start: usize,
        s: &[char],
        curr_list: &mut Vec<String>,
        memo: &mut Vec<i32>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        //If the remaining string is palidrome then we return immediately
        if start == s.len() || Self::is_palindrome(&s, start, s.len() - 1, dp) {
            return 0;
        }
        if memo[start] != -1 {
            return memo[start];
        }
        let mut mincut = i32::MAX;
        for k in start..s.len() {
            if s[start] == s[k] && (k - start < 2 || Self::is_palindrome(&s, start + 1, k - 1, dp))
            {
                dp[start][k] = 1;
                curr_list.push(String::from_iter(&s[start..(k + 1)]));
                mincut = mincut.min(
                    1 + Self::generate_palindromic_partitions_top_down(
                        k + 1,
                        s,
                        curr_list,
                        memo,
                        dp,
                    ),
                );
                curr_list.pop();
            } else {
                dp[start][k] = 0;
            }
        }
        memo[start] = mincut;
        mincut
    }

    fn generate_palindromic_partitions_DP_bu(s: &[char], dp: &mut Vec<Vec<i32>>) -> i32 {
        let mut memo = vec![0; s.len()+1];
        for i in (1..=s.len()).rev() {
            let mut mincut: i32 = i32::MAX;
            for j in i..=s.len() {
                if Self::is_palindrome(&s, i-1, j-1, dp) {
                    dp[i-1][j-1] = 1;
                    mincut = mincut.min( 1 + memo[j] );
                } else {
                    dp[i-1][j-1] = 0;
                }
            }
            memo[i-1] = mincut;
        }
        println!("----MEMO----");
        for r in memo.iter() {
            println!("{r:?}");
        }

        memo[0] - 1
    }

    fn generate_palindromic_partitions_bottom_up(s: &[char]) -> i32 {
        let mut tabulation: Vec<i32> = (0..s.len() as i32).collect();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for end in 0..s.len() {
            for start in 0..=end {
                println!("  ----{start:3}, {end:3}----");
                if s[start] == s[end] && ((end - start <= 2) || dp[start + 1][end - 1]) {
                    println!("  Is Palindrome ----{start:3}, {end:3}----");
                    dp[start][end] = true;
                    tabulation[end] = match start == 0 {
                        true => {
                            println!(
                                "      tab before {end:3} -> {} start=0 -> {}",
                                tabulation[end], 0
                            );
                            0
                        }
                        false => {
                            println!(
                                "      tab before {end:3} -> {} tab[{}] -> {} +1 {}",
                                tabulation[end],
                                start - 1,
                                tabulation[start - 1],
                                tabulation[start - 1] + 1,
                            );
                            tabulation[end].min(tabulation[start - 1] + 1)
                        }
                    };
                    println!("      tab after  {end:3} -> {}", tabulation[end]);
                }
            }
        }
        tabulation[s.len() - 1] as i32
    }

    fn min_cuts(s: &[char], mut start: usize, mut end: usize, tabulation: &mut Vec<i32>) -> i32 {
        println!(" ---- min cuts ----");
        while end < s.len() && s[start] == s[end] {
            println!("  Is Palindrome ----{start:3}, {end:3}----");
            tabulation[end] = tabulation[end].min(match start {
                0 => {
                    println!(
                        "      tab before {end:3} -> {} start=0 -> {}",
                        tabulation[end], 0
                    );
                    0
                }
                _ => {
                    println!(
                        "      tab before {end:3} -> {} tab[{}] -> {} +1 {}",
                        tabulation[end],
                        start - 1,
                        tabulation[start - 1],
                        tabulation[start - 1] + 1,
                    );
                    tabulation[start - 1] + 1
                }
            });
                    println!("      tab after  {end:3} -> {}", tabulation[end]);
            if start > 0 {
                start -= 1;
            } else {
                break;
            }

            end += 1;
        }
        0
    }

    fn expand_from_center_with_tabulation(s: &[char]) -> i32 {
        let mut tabulation: Vec<i32> = (0..s.len() as i32).collect();
        for mid in 0..s.len() {
            println!(" ----{mid}----");
            Self::min_cuts(s, mid, mid, &mut tabulation);
            Self::min_cuts(s, mid, mid + 1, &mut tabulation);
        }
        tabulation[s.len() - 1]
    }

    pub fn min_cut(s: String) -> i32 {
        //Generate palindromic_partitions
        println!("> {s}");
        let s = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![-1; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = 1;
            if i < s.len() - 1 {
                if s[i] == s[i + 1] {
                    dp[i][i + 1] = 1;
                }
            }
        }
        /*
        let mut curr_list: Vec<String> = Vec::new();
        let mut memo = vec![-1; s.len()];
        Self::generate_palindromic_partitions_top_down(0,&s, &mut curr_list, &mut memo, &mut dp) 

        //Self::generate_palindromic_partitions_iterative(&mut mincut, &s, &mut curr_list);
                match Self::generate_palindromic_partitions_bottom_up(0, &s, &mut curr_list, &mut memo, &mut dp) {
            i32::MAX => 0,
            num => num,
        }
        */
        //Self::generate_palindromic_partitions_bottom_up(&s)
        //Self::expand_from_center_with_tabulation(&s)
        Self::generate_palindromic_partitions_DP_bu(&s, &mut dp)
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
            ( "apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp".to_string(), 452),
            ( "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), 1),
        ]
    }

    #[test]
    fn test_partition() {
        for (s, want) in testcases() {
            assert_eq!(Solution::min_cut(s), want);
        }
    }
}
