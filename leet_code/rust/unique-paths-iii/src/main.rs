struct Solution {}

impl Solution {
    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        x: usize,
        y: usize,
        m: usize,
        n: usize,
        visited_cells: i32,
        empty_cells: i32,
    ) -> i32 {
        let mut total_paths = 0;
        if grid[x][y] == -1 || grid[x][y] >= 8 {
            return 0;
        }

        if grid[x][y] == 2 && visited_cells > empty_cells {
            return 1;
        }

        if grid[x][y] != -1 {
            grid[x][y] += 8;
            if x + 1 < m {
                total_paths += Self::dfs(
                    grid,
                    x + 1,
                    y,
                    m,
                    n,
                    visited_cells + 1,
                    empty_cells,
                );
            }
            if y + 1 < n {
                total_paths += Self::dfs(
                    grid,
                    x,
                    y + 1,
                    m,
                    n,
                    visited_cells + 1,
                    empty_cells,
                );
            }
            if x != 0 {
                total_paths += Self::dfs(
                    grid,
                    x - 1,
                    y,
                    m,
                    n,
                    visited_cells + 1,
                    empty_cells,
                );
            }
            if y != 0 {
                total_paths +=  Self::dfs(
                    grid,
                    x,
                    y - 1,
                    m,
                    n,
                    visited_cells + 1,
                    empty_cells,
                );
            }
            grid[x][y] -= 8;
        }
        total_paths
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let (mut x, mut y) = (0, 0);
        let mut empty_cells = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    x = i;
                    y = j;
                }
                if grid[i][j] == 0 {
                    empty_cells += 1;
                }
            }
        }
        Self::dfs(&mut grid, x, y, m, n, 0, empty_cells)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![0, 1], vec![2, 0]], 0),
            (
                vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]],
                2,
            ),
            (
                vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]],
                4,
            ),
        ]
    }

    #[test]
    fn test_unique_paths_iii() {
        for (i, (grid, res)) in testcases().into_iter().enumerate() {
            println!("Testcase #{i}");
            assert_eq!(Solution::unique_paths_iii(grid), res);
        }
    }
}
