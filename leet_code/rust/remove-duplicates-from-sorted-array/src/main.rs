/* Easy
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array
 */

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[left] != nums[right] {
                left += 1;
                nums[left] = nums[right];
            }
        }
        (left + 1) as i32
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut test_cases = vec![
            (vec![1, 1, 2], 2),
            (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5),
            (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 4], 5),
            (vec![4], 1),
        ];

        for test_case in test_cases.iter_mut() {
            assert_eq!(test_case.1, Solution::remove_duplicates(&mut test_case.0));
        }
    }
}
