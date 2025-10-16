//https://leetcode.com/problems/move-zeroes/

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l = 0;
        for i in 1..nums.len() {
            if nums[l] == 0 && nums[i] != 0 {
                nums[l] = nums[i];
                nums[i] = 0;
                l +=1;
            }
        } 
        println!("{nums:?}");
    }
    pub fn move_zeroes_pass_2(nums: &mut Vec<i32>) {
        let mut left = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[left] = nums[i];
                left += 1;
            }
        }
        while left < nums.len() {
            nums[left] = 0;
            left += 1;
        }
    }
    pub fn move_zeroes_pass_1(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = left;
        if nums.len() > 0 {
            while right < nums.len() - 1 {
                if nums[left] == 0 {
                    right += 1;
                } else {
                    left += 1;
                    right += 1;
                }
                nums[left] = nums[right];
            }
        }
        left += 1;
        while left < nums.len() {
            nums[left] = 0;
            left += 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::Solution;

    fn testcases() -> Vec<(Vec<i32>, Vec<i32>)> {
        vec![
            (vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]),
            (vec![0], vec![0]),
        ]
    }

    #[test]
    fn test_move_zeroes() {
        for (mut nums, want) in testcases().into_iter() {
            Solution::move_zeroes(&mut nums);
            assert_eq!(nums, want);
        }
    }
}
