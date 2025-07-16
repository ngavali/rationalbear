/* Medium
 * https://leetcode.com/problems/rotate-array/
 *
 * Traversing all elements and moving it to its target postition after the shift
 * Detect cycles
 *
 */

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let (k, mut N, mut shift) = (k as usize, nums.len(), 0);
        let (mut curr_pos, mut O_1) = (shift, nums[shift]);

        loop {
            loop {
                N -= 1;
                let target_pos = (curr_pos + k) % nums.len();
                (curr_pos, nums[target_pos], O_1) = (target_pos, O_1, nums[target_pos]);
                if shift == curr_pos {
                    break;
                }
            }
            shift += 1;
            if shift < k && N > 0 {
                curr_pos = shift;
                O_1 = nums[curr_pos];
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut input = vec![-1, -100, 3, 99];
    println!("{input:?}");
    Solution::rotate(&mut input, 2);
    println!("{input:?}");
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_rotate() {
        let mut test_cases = vec![
            (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
            (vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]),
        ];

        for (nums, k, expected) in test_cases.iter() {
            let mut nums = nums.clone();
            Solution::rotate(&mut nums, *k);
            for (i, _) in expected.iter().enumerate() {
                assert_eq!(expected[i], nums[i]);
            }
        }
    }
}
