//https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/
struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut cnt = 0;
        nums.sort_unstable();
        let mut prev = i32::MIN;
        if k > 0 {
            for num in nums.iter() {
                let next = prev.max(num - k);
                if next <= num + k {
                    cnt += 1;
                    prev = next + 1;
                }
            }
        } else {
            nums.dedup();
            cnt = nums.len() as i32;
        }
        cnt
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 2, 2, 3, 3, 4], 2, 6),
            (vec![4, 4, 4, 4], 1, 3),
            (vec![1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5], 3, 10),
            (vec![7, 7, 7, 7, 9], 1, 4),
            (vec![8, 8, 10, 9, 9], 1, 5),
            (vec![10, 9, 9, 8], 0, 3),
            (
                vec![
                    898100252, 691476134, 981226697, 233748869, 551821106, 140353241, 87515586,
                ],
                47249228,
                7,
            ),
            (vec![7, 10, 10], 2, 3),
        ]
    }

    use super::Solution;
    #[test]
    fn test_max_distinct_elements() {
        for (nums, k , want) in testcases() {
            assert_eq!(Solution::max_distinct_elements(nums, k), want);
        }
    }
}
