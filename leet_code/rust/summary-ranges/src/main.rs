//https://leetcode.com/problems/summary-ranges/
struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return Vec::new();
        }
        let mut ranges = Vec::with_capacity(nums.len());
        let mut curr_range = (nums[0],nums[0]);
        for i in 1..nums.len() {
            if curr_range.1 + 1 == nums[i] {
                curr_range.1 = nums[i];
            } else {
                ranges.push(if curr_range.0 == curr_range.1 {
                    format!("{}", curr_range.0)
                } else {
                    format!("{}->{}", curr_range.0, curr_range.1)
                });
                curr_range = (nums[i], nums[i]);
            }
        }
        ranges.push(if curr_range.0 == curr_range.1 {
            format!("{}", curr_range.0)
        } else {
            format!("{}->{}", curr_range.0, curr_range.1)
        });
        ranges
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    fn testcases() -> Vec<(Vec<i32>, Vec<String>)> {
        vec![
            (vec![0,1,2,4,5,7,8], vec!["0->2".to_string(),"4->5".to_string(),"7->8".to_string()]),
             (vec![0,2,3,4,6,8,9], vec!["0".to_string(),"2->4".to_string(),"6".to_string(),"8->9".to_string()]),
              (vec![], vec![]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_summary_ranges() {
        for (nums, want) in testcases() {
            assert_eq!(Solution::summary_ranges(nums), want);
        }
    }
}
