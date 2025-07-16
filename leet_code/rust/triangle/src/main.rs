/* Medium
 * https://leetcode.com/problems/triangle/
 */
struct Solution;

impl Solution {
    fn traverse_triange(
        height: usize,
        next_idx: usize,
        triangle: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if height >= triangle.len() {
            return 0;
        }
        if dp[height][next_idx] != -1 {
            return dp[height][next_idx];
        }
        let res = triangle[height][next_idx]
            + Solution::traverse_triange(height + 1, next_idx, &triangle, dp).min(
                Solution::traverse_triange(height + 1, next_idx + 1, &triangle, dp),
            );

        dp[height][next_idx] = res;
        res
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let i = 0;
        let mut dp = vec![vec![-1; triangle.len()]; triangle.len()];
        Solution::traverse_triange(0, i, &triangle, &mut dp)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_minimum_total() {
        let test_cases = vec![
            (
                vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
                11,
            ),
            (vec![vec![-10]], -10),
        ];

        for (input, output) in test_cases.into_iter() {
            assert_eq!(output, Solution::minimum_total(input));
        }
    }
}
