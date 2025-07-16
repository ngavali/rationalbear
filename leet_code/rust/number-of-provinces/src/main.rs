#![feature(test)]
/* Medium
 * https://leetcode.com/problems/number-of-provinces/
 */

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        Solution::find_circle_num_vector(is_connected)
    }

    pub fn find_circle_num_dequeue(is_connected: Vec<Vec<i32>>) -> i32 {
        let N = is_connected.len();
        let mut provinces = 0;
        let mut is_connected = is_connected;
        let mut queue = VecDeque::with_capacity(N);
        //BFS approach
        for i in 0..N {
            for j in 0..N {
                if is_connected[i][j] == 1 {
                    queue.push_back(j);
                    while !queue.is_empty() {
                        for _ in 0..queue.len() {
                            if let Some(x) = queue.pop_front() {
                                for y in 0..N {
                                    if is_connected[x][y] == 1 {
                                        queue.push_back(y);
                                        is_connected[x][y] = 2;
                                    }
                                }
                            }
                        }
                    }
                    provinces += 1;
                }
            }
        }
        provinces
    }

    pub fn find_circle_num_vector(is_connected: Vec<Vec<i32>>) -> i32 {
        let N = is_connected.len();
        let mut provinces = 0;
        let mut is_connected = is_connected;
        let mut queue = Vec::with_capacity(N);
        //BFS approach
        for i in 0..N {
            for j in 0..N {
                if is_connected[i][j] == 1 {
                    queue.push(j);
                    while !queue.is_empty() {
                        for _ in 0..queue.len() {
                            if let Some(x) = queue.pop() {
                                for y in 0..N {
                                    if is_connected[x][y] == 1 {
                                        queue.push(y);
                                        is_connected[x][y] = 2;
                                    }
                                }
                            }
                        }
                    }
                    provinces += 1;
                }
            }
        }
        provinces
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn test_cases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2),
            (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
        ]
    }

    #[test]
    fn test_find_circle_num() {
        for (input, output) in test_cases() {
            assert_eq!(output, Solution::find_circle_num(input));
        }
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_using_dequeue(b: &mut Bencher) {
        b.iter(|| {
            for (input, output) in test_cases() {
                assert_eq!(output, Solution::find_circle_num_dequeue(input));
            }
        });
    }

    #[bench]
    fn bench_using_vector(b: &mut Bencher) {
        b.iter(|| {
            for (input, output) in test_cases() {
                assert_eq!(output, Solution::find_circle_num_vector(input));
            }
        });
    }
}
