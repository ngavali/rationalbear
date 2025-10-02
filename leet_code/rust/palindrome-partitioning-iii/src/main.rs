//https://leetcode.com/problems/palindrome-partitioning-iii/

struct Solution;

impl Solution {
    fn num_char_change_to_make_palindrome(s: &[u8]) -> i32 {
        println!(" {:?}", String::from_utf8(s.to_vec()));
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut c = 0;
        while i < j {
            //println!("{s:?} ({i},{j}) -> {c}");
            if s[i] != s[j] {
                c += 1;
            }
            i += 1;
            j -= 1;
        }
        c
    }

    fn dfs(
        s: &[u8],
        i: usize,
        k: i32,
        curr_list: &mut Vec<String>,
        min_change: &mut i32,
        dp: &mut Vec<Vec<i32>>,
        memo: &mut Vec<Vec<i32>>,
        c: i32,
    ) -> i32 {
        let mut mc = i32::MAX;
        if memo[i][k as usize - 1] != i32::MAX {
            return memo[i][k as usize - 1] as i32;
        }
        if k == 1 {
            let sub_str: String = String::from_utf8(s[i..s.len()].to_vec()).unwrap();
            return Self::num_char_change_to_make_palindrome(&sub_str.as_bytes());
        }
        if k == 0 && i < s.len() {
            return mc;
        }
        if i >= s.len() && k == 0 {
            /*if c == 0 {
                println!(" B -> i={i}, k={k}, c={c}");
                println!(" Zero >>> {curr_list:?}");
            }*/
            //*min_change = c.min(*min_change);
            //println!(" >>> {curr_list:?} {min_change}");
            return 0;
        }
        if i >= s.len() {
            return i32::MAX;
        }
        /*println!("-- MEMO --");
        for r in memo.iter() {
            println!("{r:?}");
        }*/

        //println!("      Range: {} : {} -> {}",i,i+1,s.len());
        for j in i + 1..s.len() {
            let sub_str = String::from_utf8(s[i..j].to_vec()).unwrap();
            //println!("      B-> {i} {k}");
            //println!("      At Range: {} : {} -> {} {sub_str}",i,j,s.len());
            /*if dp[i][j - 1] == -1 {
                dp[i][j - 1] = Self::num_char_change_to_make_palindrome(&sub_str.as_bytes());
            }*/
            let tc = Self::num_char_change_to_make_palindrome(&sub_str.as_bytes());
            curr_list.push(sub_str);
            //let tc = c + dp[i][j - 1];
            let r = Self::dfs(s, j, k - 1, curr_list, min_change, dp, memo, c + tc);
            //println!("{mc} {tc} {r}");
            if r != i32::MAX {
                mc = mc.min(tc + r);
            }
            curr_list.pop();
        }
        //println!("{i} {k} -> {mc}");
        memo[i][k as usize - 1] = mc;
        /*println!("-- MEMO --");
        for r in memo.iter() {
            println!("{r:?}");
        }*/
        mc
    }
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        //println!("--- {s} --- {k} ---");
        let mut curr_list: Vec<String> = Vec::new();
        let mut min_change = s.len() as i32;
        let mut dp = vec![vec![-1; s.len()]; s.len()];
        let mut memo = vec![vec![i32::MAX; k as usize]; s.len()];

        let mc = Self::dfs(
            &s.as_bytes(),
            0,
            k,
            &mut curr_list,
            &mut min_change,
            &mut dp,
            &mut memo,
            0,
        );
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
