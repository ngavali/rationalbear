struct Solution {}

impl Solution {
    fn dfs(
        to_fill: usize,
        l_c: usize,
        e_c: usize,
        t_c: usize,
        memo: &mut Vec<Vec<Vec<Vec<i64>>>>,
    ) -> i64 {
        if to_fill == 0 {
            return (l_c == 1 && e_c == 2 && t_c == 1) as i64;
        }
        if memo[to_fill - 1][l_c][e_c][t_c] != 0 {
            return memo[to_fill - 1][l_c][e_c][t_c];
        }
        //Add any but l,e,t
        let count = (Self::dfs(to_fill - 1, l_c, e_c, t_c, memo) * 23) % 1000000007;
        //Add l
        let count = (count + Self::dfs(to_fill - 1, std::cmp::min(1 + l_c, 1), e_c, t_c, memo))
            % 1000000007;
        //Add e
        let count = (count + Self::dfs(to_fill - 1, l_c, std::cmp::min(1 + e_c, 2), t_c, memo))
            % 1000000007;
        //Add t
        let count = (count + Self::dfs(to_fill - 1, l_c, e_c, std::cmp::min(1 + t_c, 1), memo))
            % 1000000007;
        memo[to_fill - 1][l_c][e_c][t_c] = count;
        count
    }

    fn dfs_to_dp(
        to_fill: usize,
        l_c: usize,
        e_c: usize,
        t_c: usize,
        memo: &mut Vec<Vec<Vec<Vec<i64>>>>,
    ) -> i64 {
        memo[0][1][2][1] = 1;
        for o in 1..=to_fill {
            for l in 0..=l_c {
                for e in 0..=e_c {
                    for t in 0..=t_c {
                        let count = (memo[o - 1][l][e][t] * 23) % 1000000007;
                        let count =
                            (count + memo[o - 1][std::cmp::min(1 + l, 1)][e][t]) % 1000000007;
                        let count =
                            (count + memo[o - 1][l][std::cmp::min(1 + e, 2)][t]) % 1000000007;
                        let count =
                            (count + memo[o - 1][l][e][std::cmp::min(1 + t, 1)]) % 1000000007;
                        memo[o][l][e][t] = count;
                    }
                }
            }
        }
        println!("DP table - {:?}", memo[to_fill][0][0][0]);
        memo[to_fill][0][0][0]
    }

    pub fn string_count(n: i32) -> i32 {
        //let mut memo = vec![vec![vec![vec![0;2];3];2];n as usize];
        //Self::dfs(n as usize, 0, 0, 0, &mut memo) as i32
        let mut prev = vec![vec![vec![0 as i64; 2]; 3]; 2];
        let (mut to_fill, mut l_c, mut e_c, mut t_c) = (n as usize, 1, 2, 1);
        prev[1][2][1] = 1;
        for o in 1..=to_fill {
            let mut curr = vec![vec![vec![0 as i64; 2]; 3]; 2]; 
            for l in 0..=l_c {
                for e in 0..=e_c {
                    for t in 0..=t_c {
                        let count = (prev[l][e][t] * 23) % 1000000007;
                        let count =
                            (count + prev[std::cmp::min(1 + l, 1)][e][t]) % 1000000007;
                        let count =
                            (count + prev[l][std::cmp::min(1 + e, 2)][t]) % 1000000007;
                        let count =
                            (count + prev[l][e][std::cmp::min(1 + t, 1)]) % 1000000007;
                        curr[l][e][t] = count;
                    }
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        println!("DP table - {:?}", prev[0][0][0]);
        prev[0][0][0] as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(i32, i32)> {
        vec![(4, 12), (10, 83943898)]
    }

    #[test]
    fn test_string_count() {
        for (i, (n, res)) in testcases().into_iter().enumerate() {
            println!("Testcase #{i}");
            assert_eq!(Solution::string_count(n), res);
        }
    }
}
