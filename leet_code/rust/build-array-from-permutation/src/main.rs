struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            nums[i] = nums[i] + n * (nums[nums[i] as usize] % n);
        }
        for i in 0..nums.len() {
            nums[i] = nums[i] / n;
        }
        nums
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{

    use super::Solution;
    fn testcases()  -> Vec<(Vec<i32>, Vec<i32>)> {
        vec![
            (vec![0,2,1,5,3,4], vec![0,1,2,4,5,3]),
            (vec![5,0,1,2,3,4],vec![4,5,0,1,2,3]),
        ]
    }
    #[test]
    fn test_build_array() {
        for (nums, want) in testcases() {
            assert_eq!(Solution::build_array(nums), want);
        }
    }
}
