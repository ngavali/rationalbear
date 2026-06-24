struct Solution {}

use std::mem;
impl Solution {
    fn dfs(
        obstacle_grid: &Vec<Vec<i32>>,
        x: usize,
        y: usize,
        m: usize,
        n: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        //println!("At position: x={} y={} (m={} n={})", x, y, m, n);
        let mut res = 0;
        if (x == m || y == n || obstacle_grid[x][y] == 1) {
            return res;
        }

        if (!(x == m - 1 && y == n - 1)) {
            if memo[x][y] != -1 {
                return memo[x][y];
            }
            res += Self::dfs(obstacle_grid, x + 1, y, m, n, memo);
            res += Self::dfs(obstacle_grid, x, y + 1, m, n, memo);
            memo[x][y] = res;
            return res;
        }

        //println!("Reached !!!");
        return 1;
    }
    fn dfs_to_dp(
        obstacle_grid: &Vec<Vec<i32>>,
        x: usize,
        y: usize,
        m: usize,
        n: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let mut dp = vec![vec![0; n]; m];
        let mut prev = vec![0; n];
        for x in (0..m).rev() {
            let mut curr = vec![0; n];
            for y in (0..n).rev() {
                if (obstacle_grid[x][y] == 1) {
                    dp[x][y] = 0;
                    curr[y] = 0;
                    continue;
                }

                if (x == m - 1 && y == n - 1) {
                    dp[x][y] = 1;
                    curr[y] = 1;
                    continue;
                }
                if (x < m - 1) {
                    dp[x][y] += dp[x + 1][y];
                }
                if (y < n - 1) {
                    dp[x][y] += dp[x][y + 1];
                curr[y] += curr[y + 1];
                }
                //No x bound check required here!
                curr[y] += prev[y];
            }
            mem::swap(&mut curr,&mut prev);
        }
        //println!("{:?}", dp);
        //println!("{:?}", prev);
        //dp[0][0]
        prev[0]
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut obstacle_grid = obstacle_grid;
        for x in (0..m).rev() {
            let mut curr = vec![0; n];
            for y in (0..n).rev() {
                if (obstacle_grid[x][y] == 1) {
                    obstacle_grid[x][y] = 0;
                    continue;
                }
                if (x == m - 1 && y == n - 1) {
                    obstacle_grid[x][y] = 1;
                    continue;
                }
                if (x < m - 1) {
                    obstacle_grid[x][y] += obstacle_grid[x + 1][y];
                }
                if (y < n - 1) {
                    obstacle_grid[x][y] += obstacle_grid[x][y + 1];
                }
            }
        }
        obstacle_grid[0][0]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2),
            (vec![vec![0, 1], vec![0, 0]], 1),
            (
                vec![
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                        1, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 1,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
                        0, 1, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
                        0, 0, 0, 0, 1, 0, 0, 1,
                    ],
                    vec![
                        0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                        0, 0, 1, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 1, 1, 0,
                    ],
                    vec![
                        1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1,
                        0, 1, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 1, 1, 0, 0, 1, 0,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
                        0, 0, 1, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0,
                        1, 0, 1, 0, 0, 0, 0, 1,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
                        0, 1, 1, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
                        0, 0, 1, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 1, 0, 0, 1, 1, 0, 1,
                    ],
                    vec![
                        1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0,
                        0, 1, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1,
                        0, 1, 0, 0, 0, 0, 1, 1,
                    ],
                    vec![
                        0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        1, 0, 1, 1, 0, 1, 0, 1,
                    ],
                    vec![
                        1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    vec![
                        0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 1, 1,
                    ],
                    vec![
                        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
                        1, 0, 0, 0, 1, 0, 0, 0,
                    ],
                ],
                1637984640,
            ),
        ]
    }

    #[test]
    fn test_unique_paths_with_obstacles() {
        for (i, (obstacle_grid, res)) in testcases().into_iter().enumerate() {
            assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), res);
        }
    }
}
