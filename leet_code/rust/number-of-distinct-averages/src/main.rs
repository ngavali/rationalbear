//https://leetcode.com/problems/number-of-distinct-averages/
struct Solution;
impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let (mut i, mut j) = (0, nums.len() - 1);
        let mut avgs: Vec<i32> = Vec::new();
        while i < j {
            if let Err(idx) = avgs.binary_search_by(|a| a.cmp(&(nums[i] + nums[j]))) {
                avgs.insert(idx, nums[i] + nums[j]);
            }
            i += 1;
            j -= 1;
        }
        avgs.len() as i32
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![4, 1, 4, 0, 3, 5], 2),
            (vec![1, 100], 1),
            (vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7], 5),
        ]
    }

    use super::Solution;
    #[test]
    fn test_distinct_averages() {
        for (nums, want) in testcases() {
            assert_eq!(Solution::distinct_averages(nums), want);
        }
    }
}
