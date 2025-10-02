//https://leetcode.com/problems/palindrome-partitioning-iii/

struct Solution;

impl Solution {
    fn num_char_change_to_make_palindrome(s: &[u8], start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[start][end] == -1 {
            let mut i = start;
            let mut j = end ;
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

    fn dfs(
        s: &[u8],
        i: usize,
        k: i32,
        dp: &mut Vec<Vec<i32>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let mut mc = s.len() as i32;
        if memo[i][k as usize - 1] != 200 {
            return memo[i][k as usize - 1];
        }
        if k == 1 {
            return Self::num_char_change_to_make_palindrome(s, i, s.len()-1, dp);
        }
        for j in i + 1..s.len() {
            mc = mc.min(
                Self::num_char_change_to_make_palindrome(s, i, j-1, dp)
                + Self::dfs(s, j, k - 1, dp, memo)
            );
        }
        memo[i][k as usize - 1] = mc;
        mc
    }
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        //println!("--- {s} --- {k} ---");
        let mut dp = vec![vec![-1; s.len()]; s.len()];
        //Constraints 1 <= k <= s.length <= 100
        let mut memo = vec![vec![200; k as usize]; s.len()];

        let mc = Self::dfs(
            &s.as_bytes(),
            0,
            k,
            &mut dp,
            &mut memo,
        ) as i32;
        /*println!("mc -> {mc}");
        println!("-- MEMO --");
        for r in memo.iter() {
            println!("{r:?}");
        }*/
        mc
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
            ("babbaaaaab".to_string(), 3, 1),
            ("abc".to_string(), 2, 1),
            ("aabbc".to_string(), 3, 0),
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
            assert_eq!(Solution::palindrome_partition(s, k), want);
        }
    }
}
