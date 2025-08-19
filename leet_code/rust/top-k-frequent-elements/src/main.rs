struct Solution;

use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut num_freq = HashMap::<i32, i32>::new();
        for num in nums {
            num_freq
                .entry(num)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }
        let mut min_heap = BinaryHeap::<Reverse<(i32, i32)>>::new();

        for num in num_freq.iter() {
            min_heap.push(Reverse((*num.1, *num.0)));
            if min_heap.len() > k {
                min_heap.pop();
            };
        }

        /*
         * Using sorted vec here to simplify test case assertion
         * Not required in original solution
         */
        min_heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse((_, b))| b)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_top_k_frequent() {
        let testcases = vec![
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
        ];

        for testcase in testcases {
            assert_eq!(testcase.2, Solution::top_k_frequent(testcase.0, testcase.1));
        }
    }
}
