struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_subarray_timedout(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] >= k {
                return 1;
            } else {
                return -1;
            }
        }
        let mut min = i32::MAX;
        let mut end = 0;

        let mut sums = Vec::with_capacity(nums.len());
        sums.push(nums[0]);
        for (i, num) in nums[1..].iter().enumerate() {
            sums.push(sums[i] + num);
        }

        while end < sums.len() {
            let mut start = end;
            let mut temp_sum = nums[end];
            while start > 0 {
                if temp_sum >= k {
                    min = std::cmp::min(min, (end - start + 1) as i32);
                }
                temp_sum += nums[start - 1];
                start -= 1;
            }
            if temp_sum >= k {
                min = std::cmp::min(min, (end - start + 1) as i32);
            }
            end += 1;
        }

        match min {
            i32::MAX => -1,
            _ => min,
        }
    }

    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut sums: Vec<i64> = vec![0; nums.len() + 1];

        for (i, num) in nums.iter().enumerate() {
            if nums[i] >= k {
                return 1;
            }
            sums[i + 1] = sums[i] + *num as i64;
        }

        let mut dequeue: VecDeque<usize> = VecDeque::with_capacity(nums.len());

        for i in 0..=nums.len() {
            while dequeue
                .front()
                .is_some_and(|&j| (sums[i] - sums[j]) >= k as i64)
            {
                min = std::cmp::min(min, (i - dequeue.pop_front().unwrap()) as i32);
            }
            while dequeue.back().is_some_and(|&j| sums[j] >= sums[i]) {
                dequeue.pop_back();
            }
            dequeue.push_back(i);
        }
        match min {
            i32::MAX => -1,
            _ => min,
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test_shortest_subarray() {
        let test_cases = vec![
            (vec![-28, 81, -20, 28, -29], 89, 3),
            (vec![84, -37, 32, 40, 95], 167, 3),
            (vec![1], 1, 1),
            (vec![1, 2], 4, -1),
            (vec![2, -1, 2], 3, 3),
            (vec![77, 19, 35, 10, -14], 19, 1),
        ];

        for test_case in test_cases.into_iter() {
            println!("Testcase -> {:?}", test_case);
            assert_eq!(
                test_case.2,
                Solution::shortest_subarray(test_case.0, test_case.1)
            );
        }
    }
}
