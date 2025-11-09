//https://leetcode.com/problems/delete-greatest-value-in-each-row/

struct Solution;

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        for mut r in grid.iter_mut() {
            r.sort_unstable();
        }
        let mut mx_sum = 0;
        while grid[0].len() > 0 {
            let mut mx = 0;
            for r in grid.iter_mut() {
                if let Some(r_max) = r.pop() {
                    mx = mx.max(r_max);
                }
            }
            mx_sum += mx;
        }
        mx_sum
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![1, 2, 4], vec![3, 3, 1]], 8),
            (vec![vec![10]], 10),
        ]
    }

    use super::Solution;
    #[test]
    fn test_delete_greatest_value() {
        for (grid, want) in testcases() {
            assert_eq!(Solution::delete_greatest_value(grid), want);
        }
    }
}
