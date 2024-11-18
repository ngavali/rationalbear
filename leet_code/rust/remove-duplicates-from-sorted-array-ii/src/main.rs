/*
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 */
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = nums.len();
        if left > 2 {
            left = 2;
            for right in 2..nums.len() {
                if nums[left - 2] != nums[right] {
                    nums[left] = nums[right];
                    left += 1;
                }
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut test_cases = vec![
            (vec![1, 1, 1, 2, 2, 3], 5, vec![1, 1, 2, 2, 3]),
            (
                vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                7,
                vec![0, 0, 1, 1, 2, 3, 3],
            ),
            (vec![1], 1, vec![1]),
        ];
        for test_case in test_cases.iter_mut() {
            Solution::remove_duplicates(&mut test_case.0);
            for i in 0..test_case.1 {
                assert_eq!(test_case.0[i], test_case.2[i]);
            }
        }
    }
}
