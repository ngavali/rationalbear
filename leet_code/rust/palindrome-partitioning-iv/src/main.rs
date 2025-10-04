//https://leetcode.com/problems/palindrome-partitioning-iv/
struct Solution;
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        println!("----{s}----");
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in (0..s.len()).rev() {
            dp[i][i] = true;
            for j in i+1..s.len() {
                dp[i][j] = s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]);
                    //println!(" {i},{j} -> s[i]={} s[j]={} {}", s[i], s[j], dp[i][j]);
            }
        }

        for i in 0..s.len() - 2 {
            for j in i + 1..s.len() - 1 {
                //println!("{},{} {},{} {},{}", 0,i,i+1,j,j+1,s.len()-1);
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
