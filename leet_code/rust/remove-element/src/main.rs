/*
 * https://leetcode.com/problems/remove-element
 *
 */
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        idx as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_remove_element() {
        let mut test_cases = vec![
            (vec![3, 2, 2, 3], 3, 2, vec![2, 2]),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 1, 3, 0, 4]),
        ];
        for test_case in test_cases.iter_mut() {
            Solution::remove_element(&mut test_case.0, test_case.1);
            for i in 0..test_case.2 {
                assert_eq!(test_case.0[i], test_case.3[i]);
            }
        }
    }
}
