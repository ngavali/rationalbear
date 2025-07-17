struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut min_deque, mut max_deque) = (VecDeque::<usize>::new(), VecDeque::<usize>::new());
        let mut left = 0;
        let mut max_subarray_size = 1;
        min_deque.push_back(0);
        max_deque.push_back(0);
        
        let (mut mn_deque, mut mx_deque) = (VecDeque::<i32>::new(), VecDeque::<i32>::new());
        println!("Nums {:?}", nums);
        for (right, num) in nums.iter().enumerate() {
            if right == 0 {continue;}
            while !min_deque.is_empty() && nums[*min_deque.back().unwrap()] > *num {
                min_deque.pop_back();
                mn_deque.pop_back();
            }
            while !max_deque.is_empty() && nums[*max_deque.back().unwrap()] < *num {
                max_deque.pop_back();
                mx_deque.pop_back();
            }
            min_deque.push_back(right);
            max_deque.push_back(right);
            mn_deque.push_back(*num);
            mx_deque.push_back(*num);
            println!("mD {:?} xD {:?} Subarray size={}", min_deque, max_deque, max_subarray_size);
            println!("B -> mD {:?} xD {:?} Subarray size={}", mn_deque, mx_deque, max_subarray_size);
            //Find the diff
            while (nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()]) > limit {
                //max_deque.front() == Some(&left) || min_deque.front() == Some(&left)) {
                //println!("Limit exceeds : {} - {} = {}", nums[*max_deque.front().unwrap()] , nums[*min_deque.front().unwrap()], nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()]);
                    //Now pop from left
                if max_deque.front() <= Some(&left) {
                    max_deque.pop_front();
                    mx_deque.pop_front();
                }
                if min_deque.front() <= Some(&left) {
                    min_deque.pop_front();
                    mn_deque.pop_front();
                }
                left+=1;
            }
            if max_subarray_size < right-left+1 {
                max_subarray_size = right-left+1 ;
            }
            println!("mD {:?} xD {:?} Subarray size={}", min_deque, max_deque, max_subarray_size);
            println!("A -> mD {:?} xD {:?} Subarray size={}", mn_deque, mx_deque, max_subarray_size);
        }
        max_subarray_size as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_longest_subarray() {
        let testcases = vec![
            (vec![5,4,1,3,7,1,2,3,4,6], 10, 10),
            (vec![8, 2, 4, 7], 4, 2),
            (vec![10, 1, 2, 4, 7, 2], 5, 4),
            (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
            (vec![1,5,6,7,8,10,6,5,6], 4, 5)
        ];
        for (nums, limit, want)  in testcases {
            assert_eq!(
                want, Solution::longest_subarray(nums, limit)
            );
        }
    }
}
