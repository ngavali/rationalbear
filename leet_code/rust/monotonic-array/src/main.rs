//https://leetcode.com/problems/monotonic-array/

struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_inc = true;
        let mut i = 1;
        while i < nums.len() {
            if nums[i - 1] == nums[i] {
                i += 1;
                continue;
            }
            if nums[i - 1] < nums[i] {
                is_inc = true;
            } else {
                is_inc = false;
            }
            break;
        }
        while i < nums.len() {
            if !(is_inc && nums[i - 1] <= nums[i]) && !(!is_inc && nums[i - 1] >= nums[i]) {
                return false;
            }
            i += 1;
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, bool)>{
        vec![
            (vec![1, 2, 2, 3], true),
            (vec![6, 5, 4, 4], true),
            (vec![1, 3, 2], false),
        ]
    }

    use super::Solution;
    #[test]
    fn test_is_monotonic() {
        for (nums, want ) in testcases() {
            assert_eq!(Solution::is_monotonic(nums), want);
        }
    }
}
