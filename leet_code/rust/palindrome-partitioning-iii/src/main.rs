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
        let n = s.len();
        let s = s.as_bytes();
        let k = k as usize;
        let mut cost = vec![vec![0; n]; n];
        let mut dp = vec![vec![127; k + 1]; n + 1];
        dp[n][0] = 0;
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                cost[i][j] = cost[i + 1][j - 1] + (s[i] != s[j]) as i32
            }
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
    #[test]
    fn test_palindrome_partition() {
        for (s, k, want) in testcases() {
        //for (s, k, want) in testcases().into_iter().take(2) {
            assert_eq!(Solution::palindrome_partition(s, k), want);
        }
    }
    #[test]
    fn test_palindrome_partition_bottom_up() {
        for (s, k, want) in testcases() {
        //for (s, k, want) in testcases().into_iter().take(2) {
            assert_eq!(Solution::palindrome_partition_bottom_up(s, k), want);
        }
    }
}
