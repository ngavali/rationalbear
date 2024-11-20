/* Medium
 *
 * https://leetcode.com/problems/jump-game/
 *
 */
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jump_size = nums[0];
        let mut idx = 1;

        while jump_size > 0 && idx < nums.len() {
            println!("{idx} -> {jump_size} {}", nums[idx]);
            if nums[idx] >= jump_size {
                jump_size = nums[idx];
            } else {
                jump_size -= 1;
            }
            idx += 1;
        }
        idx == nums.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_can_jump() {
        let test_cases = vec![
            (vec![2, 3, 1, 1, 4], true),
            (vec![3, 2, 1, 0, 4], false),
            (vec![1, 1, 1, 0], true),
        ];

        for (input, output) in test_cases {
            println!("Testcase -> {input:?} {output}");
            assert_eq!(output, Solution::can_jump(input));
        }
    }
}
