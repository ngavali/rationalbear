struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;
        let mut sum = Vec::with_capacity(nums.len() - k + 1);
        let mut num_frequency = HashMap::<i64, i64>::with_capacity(100000);

        let mut num_prev_i;
        let mut num_i;
        let mut sum_result_1: i64 = 0;

        for num in &nums[0..k] {
            match num_frequency.get_mut(&(*num as i64)) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    num_frequency.insert(*num as i64, 1);
                }
            };
        }
        let mut top_x = BTreeSet::new();
        let mut bottom = BTreeSet::new();

        for (&ky, &vl) in num_frequency.iter() {
            top_x.insert((vl, ky));
        }

        while top_x.len() > x {
            bottom.insert(top_x.pop_first().unwrap());
        }

        for (freq, num) in top_x.iter() {
            sum_result_1 += *freq as i64 * *num as i64;
        }

        sum.push(sum_result_1);

        //println!( "window \t-> {:?}\ntop \t-> {top_x:?}\nbottom \t-> {bottom:?}]", &nums[0..k]);

        for i in k..nums.len() {
            num_i = nums[i] as i64;
            num_prev_i = nums[i - k] as i64;

            //println!( "----{i}----remove {num_prev_i} add {num_i}\nwindow \t-> {:?}\ntop \t-> {top_x:?}\nbottom \t-> {bottom:?}]", &nums[i - k + 1..=i]);

            match num_frequency.get_mut(&num_prev_i) {
                Some(freq) => {
                    match top_x.remove(&(*freq, num_prev_i)) {
                        true => {
                            sum_result_1 -= *freq * num_prev_i;
                        }
                        false => {
                            bottom.remove(&(*freq, num_prev_i));
                        }
                    };
                    *freq -= 1;
                    if *freq == 0 {
                        num_frequency.remove(&num_prev_i);
                    } else {
                        bottom.insert((*freq, num_prev_i));
                    }
                }
                None => {}
            };

            match num_frequency.get_mut(&num_i) {
                Some(freq) => {
                    match top_x.remove(&(*freq, num_i)) {
                        true => {
                            top_x.insert((*freq + 1, num_i));
                            sum_result_1 += num_i;
                        }
                        false => {
                            bottom.remove(&(*freq, num_i));
                            bottom.insert((*freq + 1, num_i));
                        }
                    };
                    *freq += 1;
                }
                None => {
                    num_frequency.insert(num_i, 1);
                    bottom.insert((1, num_i));
                }
            };

            while !bottom.is_empty() && top_x.len() < x {
                let (freq, num) = bottom.pop_last().unwrap();
                sum_result_1 += freq * num;
                top_x.insert((freq, num));
            }

            while top_x.first() < bottom.last() {
                let (freq, num) = top_x.pop_first().unwrap();
                sum_result_1 -= freq * num;
                bottom.insert((freq, num));
                let (freq, num) = bottom.pop_last().unwrap();
                sum_result_1 += freq * num;
                top_x.insert((freq, num));
            }

            //println!( "----{i}----remove {num_prev_i} add {num_i}\nwindow \t-> {:?}\ntop \t-> {top_x:?}\nbottom \t-> {bottom:?}]", &nums[i - k + 1..=i]);

            /*
            sum_result = 0;
            for (freq, num) in top_x.iter() {
                sum_result += *freq as i64 * *num as i64;
            }
            */

            //println!("{sum_result} {sum_result_1}");
            sum.push(sum_result_1);
        }

        sum
    }
}

fn main() {
    println!(
        "Solution : {:?}",
        Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2) //Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_find_x_sum() {
        let testcases = vec![
            (vec![10, 7, 6, 9, 8], 2, 1, vec![10, 7, 9, 9]),
            (vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2, vec![6, 10, 12]),
            (vec![3, 8, 7, 8, 7, 5], 2, 2, vec![11, 15, 15, 15, 12]),
            (
                vec![
                    1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000,
                ],
                6,
                1,
                vec![6000000000],
            ),
            (vec![7, 10, 8, 9, 10], 5, 5, vec![44]),
            (vec![2, 2, 1, 4, 1], 4, 3, vec![9, 8]),
            (vec![2, 2, 3, 3, 4, 2], 3, 3, vec![7, 8, 10, 9]),
            (vec![2, 1, 3, 4, 3], 1, 1, vec![2, 1, 3, 4, 3]),
        ];

        for (i, testcase) in testcases.iter().enumerate() {
            println!("Test case #{i}");
            assert_eq!(
                testcase.3,
                Solution::find_x_sum(testcase.0.clone(), testcase.1, testcase.2),
            )
        }
    }
}
