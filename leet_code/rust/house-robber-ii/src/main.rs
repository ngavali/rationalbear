//https://leetcode.com/problems/house-robber-ii/
struct Solution;
struct SolutionDpBottomUp;

impl Solution {
    fn dfs(i: usize, nums: &[i32], memo: &mut Vec<i32>) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if memo[i] != -1 {
            return memo[i];
        }
        if i == nums.len() - 1 {
            return nums[i];
        }
        /*
         * Skip this and jump to adjacent
         * or
         * rob this and skip adjacent
         */
        let robbed_money = Self::dfs(i + 1, nums, memo).max(nums[i] + Self::dfs(i + 2, nums, memo));
        memo[i] = robbed_money;
        robbed_money
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        let n_1 = n - 1;
        let mut memo_1: Vec<i32> = vec![-1; n];
        let mut memo_2: Vec<i32> = vec![-1; n];
        Self::dfs(0, &nums[0..n_1], &mut memo_1).max(Self::dfs(1, &nums, &mut memo_2))
    }
}
impl SolutionDpBottomUp {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let n_1 = n - 1;
        let mut memo_1: Vec<i32> = vec![0; n + 2];
        let mut memo_2: Vec<i32> = vec![0; n + 2];
        memo_1[0] = nums[n_1];
        memo_2[n_1] = nums[n_1];
        for i in (0..nums.len() - 1).rev() {
            memo_1[i] = memo_1[i + 1].max(nums[i] + memo_1[i + 2]);
        }
        for i in (1..nums.len()).rev() {
            memo_2[i] = memo_2[i + 1].max(nums[i] + memo_2[i + 2]);
        }
        memo_1[0].max(memo_2[1])
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![2, 3, 2], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![1, 2, 3], 3),
            (vec![1, 2, 1, 1], 3),
            (vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5], 16),
            (vec![200, 3, 140, 20, 10], 340),
            (vec![1], 1),
            (
                vec![
                    94, 40, 49, 65, 21, 21, 106, 80, 92, 81, 679, 4, 61, 6, 237, 12, 72, 74, 29,
                    95, 265, 35, 47, 1, 61, 397, 52, 72, 37, 51, 1, 81, 45, 435, 7, 36, 57, 86, 81,
                    72,
                ],
                2926,
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_rob() {
        for (nums, want) in testcases() {
            assert_eq!(Solution::rob(nums), want);
        }
    }
    use super::SolutionDpBottomUp;
    #[test]
    fn test_rob_dp_bottom_up() {
        for (nums, want) in testcases() {
            assert_eq!(SolutionDpBottomUp::rob(nums), want);
        }
    }
}
