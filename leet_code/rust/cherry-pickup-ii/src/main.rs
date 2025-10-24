//https://leetcode.com/problems/cherry-pickup-ii/
struct Solution;
struct SolutionDpRecursive;
struct SolutionDpRecursiveToTab;

impl SolutionDpRecursiveToTab {
    //To do - forward
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut prev = vec![vec![0; grid[0].len()]; grid[0].len()];
        let mut next = vec![vec![0; grid[0].len()]; grid[0].len()];
        for c1 in 0..m {
            for c2 in 0..m {
                next[c1][c2] = grid[n - 1][c1] + if c1 != c2 { grid[n - 1][c2] } else { 0 };
            }
        }
        for r in (0..n - 1).rev() {
            for c1 in 0..m {
                for c2 in 0..m {
                    prev[c1][c2] = grid[r][c1]
                        + if c1 != c2 { grid[r][c2] } else { 0 }
                        + vec![c1.wrapping_sub(1), c1, c1 + 1]
                            .into_iter()
                            .map(|next_c1| {
                                vec![c2.wrapping_sub(1), c2, c2 + 1]
                                    .into_iter()
                                    .map(|next_c2| {
                                        if next_c1 < m && next_c2 < m {
                                            next[next_c1][next_c2]
                                        } else {
                                            0
                                        }
                                    })
                                    .max()
                                    .unwrap_or(0)
                            })
                            .max()
                            .unwrap_or(0);
                }
            }
            std::mem::swap(&mut prev, &mut next);
        }
        next[0][m - 1]
    }
}
impl Solution {
    fn dfs(
        r: usize,
        c1: usize,
        c2: usize,
        mn: (usize, usize),
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if memo[r][c1][c2] != -1 {
            return memo[r][c1][c2];
        }
        let cherries = grid[r][c1] + if c1 != c2 { grid[r][c2] } else { 0 };
        if r == mn.0 - 1 {
            return cherries;
        }
        let mut cherries_from_onwards = 0;
        for next_c1 in vec![c1.wrapping_sub(1), c1, c1 + 1] {
            for next_c2 in vec![c2.wrapping_sub(1), c2, c2 + 1] {
                if next_c1 < mn.1 && next_c2 < mn.1 {
                    cherries_from_onwards = cherries_from_onwards.max(Self::dfs(
                        r + 1,
                        next_c1,
                        next_c2,
                        mn,
                        grid,
                        memo,
                    ));
                }
            }
        }
        memo[r][c1][c2] = cherries + cherries_from_onwards;
        memo[r][c1][c2]
    }
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = vec![vec![vec![-1; grid[0].len()]; grid[0].len()]; grid.len()];
        Self::dfs(
            0,
            0,
            grid[0].len() - 1,
            (grid.len(), grid[0].len()),
            &grid,
            &mut memo,
        )
    }
}

impl SolutionDpRecursive {
    fn dfs(
        r1_pos: (usize, usize),
        r2_pos: (usize, usize),
        mn: (usize, usize),
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if r1_pos.1 >= mn.1 || r2_pos.1 >= mn.1 {
            return 0;
        }

        if memo[r1_pos.0][r1_pos.1][r2_pos.0][r2_pos.1] != -1 {
            return memo[r1_pos.0][r1_pos.1][r2_pos.0][r2_pos.1];
        }

        let mut cherries = if r1_pos.0 == r2_pos.0 && r1_pos.1 == r2_pos.1 {
            grid[r1_pos.0][r1_pos.1]
        } else {
            grid[r1_pos.0][r1_pos.1] + grid[r2_pos.0][r2_pos.1]
        };
        if r1_pos.0 == mn.0 - 1 {
            return cherries;
        }

        let mut r1_next_positions = vec![(r1_pos.0 + 1, r1_pos.1 + 1), (r1_pos.0 + 1, r1_pos.1)];
        let mut r2_next_positions = vec![(r2_pos.0 + 1, r2_pos.1 + 1), (r2_pos.0 + 1, r2_pos.1)];
        if r1_pos.1 > 0 {
            r1_next_positions.push((r1_pos.0 + 1, r1_pos.1 - 1));
        }
        if r2_pos.1 > 0 {
            r2_next_positions.push((r2_pos.0 + 1, r2_pos.1 - 1));
        }

        let mut picked_up_cherries_from_now_onwards = 0;
        for &r2_next_pos in r2_next_positions.iter() {
            for &r1_next_pos in r1_next_positions.iter() {
                picked_up_cherries_from_now_onwards = picked_up_cherries_from_now_onwards
                    .max(Self::dfs(r1_next_pos, r2_next_pos, mn, grid, memo));
            }
        }
        cherries += picked_up_cherries_from_now_onwards;
        memo[r1_pos.0][r1_pos.1][r2_pos.0][r2_pos.1] = cherries;
        cherries
    }
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut memo =
            vec![vec![vec![vec![-1; grid[0].len()]; grid.len()]; grid[0].len()]; grid.len()];
        Self::dfs(
            (0, 0),
            (0, grid[0].len() - 1),
            (grid.len(), grid[0].len()),
            &grid,
            &mut memo,
        )
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (
                vec![
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
                    vec![0, 6, 0, 0, 0, 0, 0, 0, 6, 0],
                    vec![0, 0, 9, 0, 0, 0, 0, 3, 0, 0],
                    vec![0, 0, 0, 8, 0, 0, 5, 0, 0, 0],
                    vec![100, 0, 0, 0, 2, 3, 0, 0, 0, 100],
                ],
                227,
            ),
            (
                vec![
                    vec![13, 14, 37, 49, 64, 98, 4, 11, 47, 81],
                    vec![71, 46, 50, 50, 10, 14, 35, 35, 52, 69],
                ],
                234,
            ),
            (
                vec![
                    vec![9, 79],
                    vec![49, 85],
                    vec![54, 48],
                    vec![37, 72],
                    vec![68, 14],
                    vec![53, 30],
                    vec![65, 80],
                    vec![94, 18],
                    vec![89, 46],
                    vec![7, 93],
                ],
                1090,
            ),
            (
                vec![
                    vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 86],
                    vec![0, 52, 0, 0, 0, 0, 0, 0, 48, 0],
                    vec![0, 0, 94, 0, 0, 0, 0, 74, 0, 0],
                    vec![0, 0, 0, 98, 0, 0, 25, 0, 0, 0],
                    vec![0, 0, 0, 0, 70, 66, 0, 0, 0, 0],
                ],
                617,
            ),
            (
                vec![
                    vec![1, 0, 0, 0, 0, 0, 1],
                    vec![2, 0, 0, 0, 0, 3, 0],
                    vec![2, 0, 9, 0, 0, 0, 0],
                    vec![0, 3, 0, 5, 4, 0, 0],
                    vec![1, 0, 2, 3, 0, 0, 6],
                ],
                28,
            ),
            (
                vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]],
                24,
            ),
            (
                vec![
                    vec![63, 56, 38, 100, 40, 50, 44, 98, 27, 20],
                    vec![13, 98, 92, 31, 46, 29, 81, 2, 37, 3],
                    vec![75, 5, 46, 15, 74, 17, 34, 60, 100, 44],
                    vec![8, 4, 17, 14, 60, 77, 23, 0, 93, 83],
                    vec![41, 40, 5, 2, 73, 80, 71, 100, 82, 39],
                    vec![96, 76, 56, 42, 65, 65, 22, 11, 85, 80],
                    vec![64, 71, 27, 78, 85, 15, 2, 28, 75, 31],
                    vec![51, 16, 30, 65, 54, 68, 12, 5, 48, 1],
                    vec![100, 57, 93, 43, 40, 51, 3, 82, 46, 91],
                    vec![81, 63, 20, 12, 83, 59, 59, 46, 67, 66],
                ],
                1400,
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_cherry_pickup() {
        for (grid, want) in testcases() {
            assert_eq!(Solution::cherry_pickup(grid), want);
        }
    }
    use super::SolutionDpRecursive;
    #[test]
    fn test_cherry_pickup_dp_recursive() {
        for (grid, want) in testcases() {
            assert_eq!(SolutionDpRecursive::cherry_pickup(grid), want);
        }
    }
    use super::SolutionDpRecursiveToTab;
    #[test]
    fn test_cherry_pickup_dp_recursive_to_tab() {
        for (grid, want) in testcases() {
            assert_eq!(SolutionDpRecursiveToTab::cherry_pickup(grid), want);
        }
    }
}
