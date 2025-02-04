/* Hard
 * https://leetcode.com/problems/sliding-window-maximum/
 * Use Monotonic Queue (Dequeue)
 */
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut max_nums = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            //Remove out of window indexes
            if !queue.is_empty() && queue.front().unwrap() + k == i {
                queue.pop_front();
            }
            //If this is max then clear everything
            if !queue.is_empty() && nums[i] > nums[*queue.front().unwrap()] {
                queue.clear();
            } else {
                //If Not the max then start cleaning up lower values
                while !queue.is_empty() && nums[i] > nums[*queue.back().unwrap()] {
                    queue.pop_back();
                }
            }
            queue.push_back(i);
            max_nums.push(nums[*queue.front().unwrap()]);
        }
        max_nums[k - 1..].to_vec()
    }
}

fn main() {
    for (i, (nums, k, exp_op)) in testcases().into_iter().enumerate() {
        println!("Testcase#{i}");
        let got_op = Solution::max_sliding_window(nums, k);
        assert_eq!(exp_op.len(), got_op.len());
        for idx in 0..exp_op.len() {
            assert_eq!(exp_op[idx], got_op[idx]);
        }
    }
}

fn testcases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (vec![9, 10, 9, -7, -4, -8, 2, -6], 5, vec![10, 10, 9, 2]),
        (vec![1, 3, 1, 2, 0, 5], 3, vec![3, 3, 2, 5]),
        (vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7]),
        (vec![1], 1, vec![1]),
        (vec![1, -1], 1, vec![1, -1]),
        (vec![7, 2, 4], 2, vec![7, 4]),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_max_sliding_window() {
        for (i, (nums, k, exp_op)) in testcases().into_iter().enumerate() {
            println!("Testcase#{i}");
            let got_op = Solution::max_sliding_window(nums, k);
            assert_eq!(exp_op.len(), got_op.len());
            for idx in 0..exp_op.len() {
                assert_eq!(exp_op[idx], got_op[idx]);
            }
        }
    }
}
