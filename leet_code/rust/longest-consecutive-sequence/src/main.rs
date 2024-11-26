/* Hard
 * https://leetcode.com/problems/longest-consecutive-sequence/
 *
 * Todo: Use HashMap to look up next or prev number. No need to sort actually
 */
struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut seq = BTreeSet::new();
        for num in nums {
            seq.insert(num);
        }
        let (mut max_length, mut curr_counter) = (1, 1);
        let mut prev = seq.pop_first().unwrap();
        for &num in seq.iter() {
            if prev + 1 != num {
                max_length = max_length.max(curr_counter);
                curr_counter = 0;
            }
            prev = num;
            curr_counter += 1;
        }
        max_length.max(curr_counter)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_longest_consecutive() {
        let test_cases = vec![
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];
        for (i, (input, output)) in test_cases.into_iter().enumerate() {
            println!("Testcase: {i}");
            assert_eq!(output, Solution::longest_consecutive(input))
        }
    }
}
