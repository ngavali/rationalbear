//https://leetcode.com/problems/palindrome-partitioning-iv/
struct Solution;
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

    fn generate_palindromic_partitions(
        start: usize,
        s: &[char],
        curr_list: &mut Vec<String>,
        possible: &mut bool,
        memo: &mut Vec<Vec<i32>>,
    ) {
        if curr_list.len() > 3 {
            return;
        }
        if start >= s.len() || *possible {
            if curr_list.len() == 3 {
                *possible = true;
            }
            return;
        }
        for k in start..s.len() {
            if Self::is_palindrome(s, start, k) && !*possible {
                curr_list.push(String::from_iter(&s[start..(k + 1)]));
                Self::generate_palindromic_partitions(k + 1, s, curr_list, possible, memo);
                curr_list.pop();
            }
        }
    }

    pub fn check_partitioning_dp(s: String) -> bool {
        //Generate palindromic_partitions
        let s: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut curr_list: Vec<String> = Vec::new();
        let mut p = false;
        let mut memo = vec![vec![-1; 4]; s.len()];
        Self::generate_palindromic_partitions(0, &s, &mut curr_list, &mut p, &mut memo);
        p
    }

    pub fn check_partitioning_dp_bottom_up(s: String) -> bool {
        println!("---{s}---");
        let n = s.len();
        let s: Vec<char> = s.chars().collect();

        let mut pal = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if s[i] == s[j] && (j - i < 2 || pal[i + 1][j - 1]) {
                    pal[i][j] = true;
                }
            }
        }

        let mut dp = vec![vec![false; 4]; n + 1];
        dp[n][0] = true;

        for i in (0..n).rev() {
            for k in 1..=3 {
                for j in i..n {
                    if pal[i][j] && dp[j + 1][k - 1] {
                        dp[i][k] = true;
                        break;
                    }
                }
            }
        }

        dp[0][3]
    }

    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        /*
         * Notice the relationship
         * i,j depends on next i and previous j
         *
         * we need to have next i computed already
         * dp[i][j] = s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1])
         *
         */
        for i in (0..s.len()).rev() {
            dp[i][i] = true;
            for j in i + 1..s.len() {
                dp[i][j] = s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]);
            }
        }

        for i in 0..s.len() - 2 {
            for j in i + 1..s.len() - 1 {
                if dp[0][i] && dp[i + 1][j] && dp[j + 1][s.len() - 1] {
                    return true;
                }
            }
        }
        false
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(String, bool)> {
        vec![
            ("ypyzztbnhlhavybskfdigqgidfksbyvahlhnbtzzypykutmtfoeeevzzvsqsvzzveeeoftmtuktsficivikrbauwhxpcjjteetjjcpxhwuabrkivicifst".to_string(), true ),
            ("abcbdd".to_string(), true),
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaxyaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
             false),
        ]
    }

    use super::Solution;

    #[test]
    fn test_check_partitioning() {
        for (s, want) in testcases() {
            assert_eq!(Solution::check_partitioning(s), want);
        }
    }
    #[test]
    fn test_check_partitioning_dp_bottom_up() {
        for (s, want) in testcases() {
            assert_eq!(Solution::check_partitioning_dp_bottom_up(s), want);
        }
    }
}

/*
 * Explore other solution
 *
fn pre(s: &[u8]) -> Vec<Option<Vec<usize>>> {
  let mut dp: Vec<Option<Vec<usize>>> = (0..s.len())
    .map(|_| None)
    .collect();

  dp[s.len() - 1] = Some(vec![s.len()]);

  for i in (0..s.len() - 1).rev() {
    let prev_dp = dp[i + 1].as_ref().unwrap();
    let mut this_dp = Vec::with_capacity(prev_dp.len() + 1);
    this_dp.push(i + 1);
    if s[i] == s[i + 1] {
      this_dp.push(i + 2);
    }
    for end in prev_dp {
      if *end < s.len() && s[i] == s[*end] {
        this_dp.push(*end + 1);
      }
    }
    dp[i] = Some(this_dp);
  }

  dp
}

fn solve(s: &[u8]) -> bool {
  let dp = pre(s);

  for partition1 in dp[0].as_ref().unwrap() {
    if *partition1 == s.len() {
      continue;
    }
    for partition2 in dp[*partition1].as_ref().unwrap() {
      if *partition2 == s.len() {
        continue;
      }
      if dp[*partition2].as_ref().unwrap().contains(&s.len()) {
        return true;
      }
    }
  }

  false
}

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
      solve(s.as_bytes())
    }
}
 *
 *
 */
