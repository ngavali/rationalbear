//https://leetcode.com/problems/palindrome-partitioning-iii/

struct Solution;

impl Solution {
    fn num_char_change_to_make_palindrome(
        s: &[u8],
        start: usize,
        end: usize,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if dp[start][end] == -1 {
            let mut i = start;
            let mut j = end;
            let mut c = 0;
            while i < j {
                if s[i] != s[j] {
                    c += 1;
                }
                i += 1;
                j -= 1;
            }
            dp[start][end] = c;
        }
        dp[start][end]
    }

    fn dfs(s: &[u8], i: usize, k: usize, dp: &mut Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
        let mut mc = s.len() as i32;
        if memo[i][k - 1] != 127 {
            return memo[i][k - 1];
        }
        if k == 1 {
            return Self::num_char_change_to_make_palindrome(s, i, s.len() - 1, dp);
        }
        for j in i + 1..s.len() {
            mc = mc.min(
                Self::num_char_change_to_make_palindrome(s, i, j - 1, dp)
                    + Self::dfs(s, j, k - 1, dp, memo),
            );
        }
        memo[i][k - 1] = mc;
        mc
    }

    pub fn palindrome_partition_bottom_up(s: String, k: i32) -> i32 {
        println!("---{s} {k}---");
        let n = s.len();
        let s = s.as_bytes();
        let k = k as usize;
        let mut cost = vec![vec![0; n]; n];
        let mut dp = vec![vec![127; k + 1]; n + 1];
        dp[n][0] = 0;
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                println!(
                    "--Before - {i:2},{j:2} - palindrome cost DP table -- res -> {} != {} -> {}",
                    s[i],
                    s[j],
                    s[i] != s[j]
                );
                for r in cost.iter() {
                    println!("{r:?}");
                }
                cost[i][j] = cost[i + 1][j - 1] + (s[i] != s[j]) as i32;
                println!("--After - {i:2},{j:2} - palindrome cost DP table --");
                for r in cost.iter() {
                    println!("{r:?}");
                }
            }
        }
        println!("--Final palindrome cost DP table --");
        for r in cost.iter() {
            println!("{r:?}");
        }
        for i in (0..n).rev() {
            dp[i][1] = cost[i][n - 1];
            for part in 2..=k {
                for j in i..n {
                    dp[i][part] = dp[i][part].min(cost[i][j] + dp[j + 1][part - 1]);
                }
            }
        }
        dp[0][k]
    }

    fn palindrome_partition_tabulation(s: String, k: i32) -> i32 {
        println!(">>> TABULATION --- {s} - {k} ---");
        let s = s.as_bytes();
        let mut cost = vec![vec![0; s.len()]; s.len()];
        for i in (0..s.len()).rev() {
            for j in i + 1..s.len() {
                println!(
                    "--Before - {i:2},{j:2} - palindrome cost cost table -- res -> {} != {} -> {}",
                    s[i],
                    s[j],
                    s[i] != s[j]
                );
                for r in cost.iter() {
                    println!("{r:?}");
                }
                cost[i][j] = (s[i] != s[j]) as i32;
                if i + 1 < j {
                    cost[i][j] += cost[i + 1][j - 1];
                }
                println!(
                    "--After  - {i:2},{j:2} - palindrome cost cost table -- res -> {} != {} -> {}",
                    s[i],
                    s[j],
                    s[i] != s[j]
                );
                for r in cost.iter() {
                    println!("{r:?}");
                }
            }
        }
        let n = s.len();
        let k = k as usize;
        let mut dp = vec![vec![0; n + 1]; k + 1];
        for length in 1..=n {
            for partition in 1..=length.min(k as usize) {
                println!(">> Before --- DP partition={partition:2}, length={length:2} ---");
                for r in dp.iter() {
                    println!("{r:?}");
                }
                if partition == 1 {
                    dp[partition][length] = cost[0][length - 1];
                } else {
                    dp[partition][length] = 127;
                    for split_point in partition - 1..length {
                        println!("\t--- DP par={partition:2},len={length:2} --- {split_point} dp[partition-1={:2}, split_point={:2}] + cost[split_point={:2}, length-1={:2}]", partition - 1, split_point, split_point, length-1);
                        println!("\t\t min of ({:3} and dp(par-1)={} + cost={}  ) ", dp[partition][length], dp[partition - 1][split_point], cost[split_point][length - 1]);
                        for r in dp.iter() {
                            println!("\t{r:?}");
                        }
                        dp[partition][length] = dp[partition][length]
                            .min(dp[partition - 1][split_point] + cost[split_point][length - 1]);
                    }
                }
                println!(">> After  --- DP {partition:2},{length:2} ---");
                for r in dp.iter() {
                    println!("{r:?}");
                }
            }
        }
        println!("--- DP length k---");
        for r in dp.iter() {
            println!("{r:?}");
        }
        dp[k][n]
    }

    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let mut dp = vec![vec![-1; s.len()]; s.len()];
        //Constraints 1 <= k <= s.length <= 100
        let mut memo = vec![vec![127; k as usize]; s.len()];
        Self::dfs(&s.as_bytes(), 0, k as usize, &mut dp, &mut memo) as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::Solution;
    fn testcases() -> Vec<(String, i32, i32)> {
        vec![
            ("aabbdd".to_string(), 3, 0),
            ("aabbc".to_string(), 3, 0),
            ("abc".to_string(), 2, 1),
            ("babbaaaaab".to_string(), 3, 1),
            ("leetcode".to_string(), 8, 0),
            (
                "asjkdnaksjdnviasdivbasbdvbasdfasdfaisdfasdfasdfasdsfffffffffffffffffsdsddddd"
                    .to_string(),
                5,
                15,
            ),
            (
                "asjkdnaksjdnviasdivbasbdvbasdfasdfaisdfasdfasdfasdsfffffffffffffffffsdsddddd"
                    .to_string(),
                20,
                8,
            ),
            ("zcfxvwnafsrczwpdyxkvutnqduortfzjgzpbjrzagsfbhaejcsepiraqfpgvcoyhsorwzwfncmoitdopbhecmbjejpldcvfwubfn".to_string(), 20, 28),
        ]
    }
    /*
    #[test]
    fn test_palindrome_partition() {
        for (s, k, want) in testcases() {
            //for (s, k, want) in testcases().into_iter().take(2) {
            assert_eq!(Solution::palindrome_partition(s, k), want);
        }
    }
    #[test]
    fn test_palindrome_partition_bottom_up() {
        //for (s, k, want) in testcases() {
        for (s, k, want) in testcases().into_iter().take(1) {
            assert_eq!(Solution::palindrome_partition_bottom_up(s, k), want);
        }
    }
    */
    #[test]
    fn test_palindrome_partition_tabulation() {
        //for (s, k, want) in testcases() {
        for (s, k, want) in testcases().into_iter().take(1) {
            assert_eq!(Solution::palindrome_partition_tabulation(s, k), want);
        }
    }
}
