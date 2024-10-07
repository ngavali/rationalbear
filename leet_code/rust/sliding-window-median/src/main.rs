use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Storage {
    left: BinaryHeap<i64>,
    right: BinaryHeap<Reverse<i64>>,
}

struct Solution;
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
        let k = k as usize;

        let mut balance = 0;

        let mut to_remove = HashMap::<i64, i64>::new();
        let mut storage = Storage {
            left: BinaryHeap::with_capacity(100000),
            right: BinaryHeap::with_capacity(100000),
        };
        let mut i = 0;

        //Populate the heap first
        while i < k {
            storage.left.push(nums[i] as i64);
            i += 1;
        }

        while i > (k + 1) / 2 {
            storage.right.push(Reverse(storage.left.pop().unwrap()));
            i -= 1;
        }

        let odd = k % 2 != 0;

        result.push(match odd {
            true => *storage.left.peek().unwrap() as f64,
            false => {
                (*storage.left.peek().unwrap() as f64 + storage.right.peek().unwrap().0 as f64)
                    / 2.0
            }
        });

        let mut median;
        i = k;

        while i < nums.len() {
            median = result[i - k];
            to_remove
                .entry(nums[i - k] as i64)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            //Old num was removed from left or right
            balance = match nums[i - k] as f64 <= median {
                true => -1,
                false => 1,
            };

            //New num was added to left or right
            if nums[i] <= median as i32 {
                storage.left.push(nums[i] as i64);
                balance += 1;
            } else {
                storage.right.push(Reverse(nums[i] as i64));
                balance -= 1;
            }

            if balance > 0 {
                storage.right.push(Reverse(storage.left.pop().unwrap()));
            } else if balance < 0 {
                storage.left.push(storage.right.pop().unwrap().0);
            }

            //Pop if remove num on top
            while let Some(k) = storage.left.peek() {
                match to_remove.get_mut(k) {
                    Some(v) => {
                        if *v > 0 {
                            *v -= 1;
                            storage.left.pop();
                        } else if *v == 0 {
                            to_remove.remove(&k);
                        }
                    }
                    _ => break,
                }
            }
            //Pop if remove num on top
            while let Some(k) = storage.right.peek() {
                match to_remove.get_mut(&k.0) {
                    Some(v) => {
                        if *v > 0 {
                            storage.right.pop();
                            *v -= 1;
                        } else if *v == 0 {
                            to_remove.remove(&k.0);
                        }
                    }
                    _ => break,
                }
            }

            result.push(match odd {
                true => *storage.left.peek().unwrap() as f64,
                false => {
                    (*storage.left.peek().unwrap() as f64 + storage.right.peek().unwrap().0 as f64)
                        / 2.0
                }
            });
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_median_sliding_window() {
        let testcases = vec![
            (
                vec![7, 9, 3, 8, 0, 2, 4, 8, 3, 9],
                1,
                vec![7.0, 9.0, 3.0, 8.0, 0.0, 2.0, 4.0, 8.0, 3.0, 9.0],
            ),
            (
                vec![1, 3, -1, -3, 5, 3, 6, 7],
                3,
                vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0],
            ),
            (
                vec![1, 2, 3, 4, 2, 3, 1, 4, 2],
                3,
                vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0],
            ),
            (vec![1, 2], 1, vec![1.0, 2.0]),
            (vec![2147483647, 2147483647], 2, vec![2147483647.0]),
            (
                vec![7, 8, 8, 3, 8, 1, 5, 3, 5, 4],
                3,
                vec![8.0, 8.0, 8.0, 3.0, 5.0, 3.0, 5.0, 4.0],
            ),
        ];
        for (i, testcase) in testcases.iter().enumerate() {
            println!("Testcase: {:3}", i);
            assert_eq!(
                Solution::median_sliding_window(testcase.0.clone(), testcase.1),
                testcase.2
            );
        }
    }
}
