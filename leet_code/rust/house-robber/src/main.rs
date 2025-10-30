//https://leetcode.com/problems/house-robber/

struct Solution;
struct SolutionDpBottomUp;

impl Solution {
    fn dfs(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
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
        let mut memo: Vec<i32> = vec![-1; nums.len()];
        Self::dfs(0, &nums, &mut memo)
    }
}

impl SolutionDpBottomUp {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.push(0);
        nums.push(0);
        for i in (0..nums.len()-2).rev() {
            nums[i] = nums[i + 1].max(nums[i] + nums[i + 2]);
        }
        nums[0]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (vec![2, 1, 1, 2], 4),
            (vec![40, 2, 4, 10], 50),
            (vec![1, 3, 5, 9], 12),
            (vec![2], 2),
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
    fn test_rob_dp_bottomt_up() {
        for (nums, want) in testcases() {
            assert_eq!(SolutionDpBottomUp::rob(nums), want);
        }
    }

}
