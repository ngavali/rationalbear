struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

const M: i64 = 1_000_000_007;

impl Solution {
    fn power(mut x: i64, mut times: i64) -> i64 {
        let mut num = 1;
        while times > 0 {
            if times & 0x1 == 0x1 {
                num = (num * x) % M;
            }
            x = (x * x) % M;
            times = times >> 1;
        }

        num
    }
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        let (mut max_num, mut k, mut multiplier) = (0, k as i64, multiplier as i64);
        let mut queue = BinaryHeap::with_capacity(nums.len());
        let mut nums: Vec<i64> = nums
            .into_iter()
            .enumerate()
            .map(|(pos, num)| {
                if max_num < num {
                    max_num = num;
                }
                queue.push(Reverse((num as i64, pos)));
                num as i64
            })
            .collect();

        let bound = max_num as i64 / multiplier;
        while k > 0 && bound >= queue.peek().unwrap().0 .0 {
            let (num, pos) = queue.pop().unwrap().0;
            nums[pos] = num * multiplier;
            queue.push(Reverse((nums[pos], pos)));
            k -= 1;
        }

        let mut rem = k % nums.len() as i64;
        let multiplier_reg = Solution::power(multiplier, k / nums.len() as i64);
        multiplier = (multiplier_reg * multiplier) % M;

        queue
            .into_sorted_vec()
            .iter()
            .rev()
            .for_each(|Reverse((num, pos))| {
                match rem > 0 {
                    true => {
                        rem -= 1;
                        nums[*pos] = (num * multiplier) % M;
                    }
                    false => nums[*pos] = (num * multiplier_reg) % M,
                };
            });

        nums.iter().map(|a| (*a % M) as i32).collect()
    }
}

fn main() {
    println!("{:?}", Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
    println!(
        "{:?}",
        Solution::get_final_state(vec![100000, 2000], 2, 1000000)
    );
    println!(
        "{:?}",
        Solution::get_final_state(vec![161209470], 56851412, 39846)
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_get_final_state() {
        let testcases = vec![
            (vec![2, 1, 3, 5, 6], 5, 2, vec![8, 4, 6, 5, 6]),
            (vec![100000, 2000], 2, 1000000, vec![999999307, 999999993]),
            (vec![161209470], 56851412, 39846, vec![198168519]),
        ];
        for testcase in testcases {
            assert_eq!(
                testcase.3,
                Solution::get_final_state(testcase.0, testcase.1, testcase.2)
            );
        }
    }
}
