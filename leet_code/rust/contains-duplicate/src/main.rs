//https://leetcode.com/problems/contains-duplicate/
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_set = HashSet::with_capacity(nums.len());
        for x in nums {
            if num_set.contains(&x) {
                return true;
            }
            num_set.insert(x);
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  fn testcases() -> Vec<(Vec<i32>, bool)> {
      vec![
          (vec![-1,2,3,1], false),
          (vec![1,2,3,4], false),
          (vec![1,1,1,3,3,4,3,2,4,2], true)
      ]
  }

use super::Solution;
  #[test]
  fn test_ () {
      for ( nums, want ) in testcases() {
          assert_eq!(Solution::contains_duplicate(nums), want);
      }
  }
}
