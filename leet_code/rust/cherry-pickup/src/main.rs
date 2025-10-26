//https://leetcode.com/problems/cherry-pickup/
struct Solution;
struct SolutionBt;
struct SolutionDp;

impl SolutionDp {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        //Contraints 1 <= n <= 50, max is 2 * 50 - 2 steps, so twice is 198 pickups max
        let mut prev = vec![vec![-200; grid.len()]; grid.len()];
        prev[0][0] = grid[0][0];
        for step in 1..=2 * n - 2 {
            let l = if step >= n { step - n + 1 } else { 0 };
            let mut next = vec![vec![i32::MIN; n]; n];
            for r1 in l..n.min(step + 1) {
                for r2 in l..n.min(step + 1) {
                    let (c1, c2) = (step - r1, step - r2);
                    if grid[r1][c1] == -1 || grid[r2][c2] == -1 {
                        continue;
                    }
                    let mut cherries_picked = grid[r1][c1];
                    if r1 != r2 {
                        cherries_picked += grid[r2][c2];
                    }
                    let prev_picked_cherries = prev[r1][r2].max(
                    if r1 > 0 {
                        prev[r1 - 1][r2].max(
                            if r2 > 0 {
                                prev[r1 - 1][r2 - 1]
                            } else {
                                -200
                            }
                        )
                    } else {
                        -200
                    }).max(
                    if r2 > 0 {
                        prev[r1][r2 - 1]
                    } else {
                        -200
                    });
                    next[r1][r2] = cherries_picked + prev_picked_cherries;
                }
            }
            std::mem::swap(&mut prev, &mut next);
        }
        match prev[n-1][n-1] {
            res if res > 0 => res,
            _ => 0,
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

    use super::SolutionDp;
    #[test]
    fn test_cherry_pickup_dp() {
        for (grid, want) in testcases().into_iter() {
            assert_eq!(SolutionDp::cherry_pickup(grid), want);
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
