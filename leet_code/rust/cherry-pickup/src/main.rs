//https://leetcode.com/problems/cherry-pickup/
struct Solution;
struct SolutionBt;
struct SolutionDpToTab;

impl SolutionDpToTab {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let n_1 = n - 1;
        let m_1 = m - 1;
        let mut dp = vec![vec![vec![i32::MIN; n]; m]; n];

        for r1 in (0..n).rev() {
            for c1 in (0..m).rev() {
                for r2 in (0..n).rev() {
                    if r1 + c1 < r2 {
                        continue;
                    }
                    let c2 = r1 + c1 - r2;
                    if c2 >= m || grid[r1][c1] == -1 || grid[r2][c2] == -1 {
                        continue;
                    }
                    if r1 == n_1 && c1 == m_1 {
                        dp[r1][c1][r2] = grid[r1][c1];
                        continue;
                    }
                    let mut cc = i32::MIN;
                    if r1 < n_1 {
                        if r2 < n_1 {
                            cc = cc.max(dp[r1 + 1][c1][r2 + 1]);
                        }
                        cc = cc.max(dp[r1 + 1][c1][r2]);
                    }
                    if c1 < m_1 {
                        if r2 < m_1 {
                            cc = cc.max(dp[r1][c1 + 1][r2 + 1]);
                        }
                        cc = cc.max(dp[r1][c1 + 1][r2]);
                    }
                    dp[r1][c1][r2] = cc + grid[r1][c1];
                    if r1 != r2 {
                        dp[r1][c1][r2] += grid[r2][c2];
                    }
                }
            }
        }
        if dp[0][0][0] > 0 {
            dp[0][0][0]
        } else {
            0
        }
    }
}

impl SolutionBt {
    fn move_back(row: usize, col: usize, grid: &mut Vec<Vec<i32>>, cc: i32, max_cc: &mut i32) {
        if grid[row][col] == -1 {
            return;
        }
        let c_c = grid[row][col];
        grid[row][col] = 0;
        if row == 0 && col == 0 {
            *max_cc = (cc + c_c).max(*max_cc);
            return;
        }
        if row > 0 {
            Self::move_back(row - 1, col, grid, cc + c_c, max_cc);
        }
        if col > 0 {
            Self::move_back(row, col - 1, grid, cc + c_c, max_cc);
        }
        grid[row][col] = c_c;
    }
    fn bt(row: usize, col: usize, grid: &mut Vec<Vec<i32>>, cc: i32, max_cc: &mut i32) {
        if grid[row][col] == -1 {
            return;
        }
        if row == grid.len() - 1 && col == grid[0].len() - 1 {
            Self::move_back(row, col, grid, cc, max_cc);
            return;
        }
        let c_c = grid[row][col];
        grid[row][col] = 0;
        if row < grid.len() - 1 {
            Self::bt(row + 1, col, grid, cc + c_c, max_cc);
        }
        if col < grid.len() - 1 {
            Self::bt(row, col + 1, grid, cc + c_c, max_cc);
        }
        grid[row][col] = c_c;
    }
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max_cc = 0;
        Self::bt(0, 0, &mut grid, 0, &mut max_cc);
        max_cc
    }
}
impl Solution {
    fn dfs(
        r1: usize,
        c1: usize,
        r2: usize,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        let c2 = r1 + c1 - r2;
        if r1 >= grid.len()
            || r2 >= grid.len()
            || c1 >= grid[0].len()
            || c2 >= grid[0].len()
            || grid[r1][c1] == -1
            || grid[r2][c2] == -1
        {
            return i32::MIN;
        }
        if r1 == grid.len() - 1 && c1 == grid[0].len() - 1 {
            return grid[r1][c1];
        }

        if memo[r1][c1][r2] != -1 {
            return memo[r1][c1][r2];
        }
        let mut cherries_pickedup = grid[r1][c1];
        if r1 != r2 {
            cherries_pickedup += grid[r2][c2];
        }

        let mut cherries_picked = i32::MIN;
        for next_pos in vec![
            (r1 + 1, c1, r2 + 1),
            (r1 + 1, c1, r2),
            (r1, c1 + 1, r2),
            (r1, c1 + 1, r2 + 1),
        ] {
            cherries_picked =
                cherries_picked.max(Self::dfs(next_pos.0, next_pos.1, next_pos.2, grid, memo));
        }
        cherries_pickedup += cherries_picked;
        memo[r1][c1][r2] = cherries_pickedup;
        cherries_pickedup
    }
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 && grid[0].len() == 1 {
            return grid[0][0];
        }
        let mut memo = vec![vec![vec![-1; grid.len()]; grid[0].len()]; grid.len()];
        //let mut max_cc = 0;
        //Self::bt(0, 0, &mut grid, 0, &mut max_cc);
        //max_cc
        match Self::dfs(0, 0, 0, &grid, &mut memo) {
            n if n > 0 => n,
            _ => 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]], 5),
            (vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]], 0),
            (
                vec![
                    vec![1, 1, 1, 1, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 1],
                    vec![1, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 1, 1, 1, 1],
                ],
                15,
            ),
            (
                vec![
                    vec![0, 1, 1, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![-1, 1, 1, 1, -1],
                    vec![0, 1, 1, 1, 0],
                    vec![1, 0, -1, 0, 0],
                ],
                11,
            ),
            (vec![vec![1]], 1),
            (vec![vec![0]], 0),
            //TLE
            (
                vec![
                    vec![1, 1, 1, 1, -1, -1, -1, 1, 0, 0],
                    vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                    vec![0, 0, 1, 1, 1, 1, 0, 1, 1, 1],
                    vec![1, 1, 0, 1, 1, 1, 0, -1, 1, 1],
                    vec![0, 0, 0, 0, 1, -1, 0, 0, 1, -1],
                    vec![1, 0, 1, 1, 1, 0, 0, -1, 1, 0],
                    vec![1, 1, 0, 1, 0, 0, 1, 0, 1, -1],
                    vec![1, -1, 0, 1, 0, 0, 0, 1, -1, 1],
                    vec![1, 0, -1, 0, -1, 0, 0, 1, 0, 0],
                    vec![0, 0, -1, 0, 1, 0, 1, 0, 0, 1],
                ],
                22,
            ),
        ]
    }
    use super::SolutionBt;
    #[test]
    fn test_cherry_pickup_bt() {
        for (grid, want) in testcases().into_iter().take(6) {
            assert_eq!(SolutionBt::cherry_pickup(grid), want);
        }
    }
    use super::SolutionDpToTab;
    #[test]
    fn test_cherry_pickup_bt_to_tab() {
        for (grid, want) in testcases().into_iter().take(6) {
            assert_eq!(SolutionDpToTab::cherry_pickup(grid), want);
        }
    }

    use super::Solution;
    #[test]
    fn test_cherry_pickup() {
        for (grid, want) in testcases() {
            assert_eq!(Solution::cherry_pickup(grid), want);
        }
    }
}
