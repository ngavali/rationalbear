//https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
struct Solution;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    ts: u32,
    x: usize,
    y: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.ts.cmp(&self.ts)
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let m = move_time.len();
        let n = move_time[0].len();
        let mut pq = BinaryHeap::<Node>::with_capacity(m*n);
        pq.push(Node {
            ts: 0,
            x: 0,
            y: 0,
        });
        let mut time = vec![vec![u32::MAX; n]; m];
        time[0][0] = 0;
        while let Some(node) = pq.pop() && node.x != m && node.y != n {
            for (n_x, n_y) in vec![
                (node.x, node.y + 1),
                (node.x + 1, node.y),
                (node.x.wrapping_sub(1), node.y),
                (node.x, node.y.wrapping_sub(1)),
            ] {
                if n_x < m && n_y < n {
                    let min_wait_time = 1 + (move_time[n_x][n_y] as u32).max(time[node.x][node.y]);
                    if time[n_x][n_y] > min_wait_time {
                        time[n_x][n_y] = min_wait_time;
                        pq.push(Node {
                            ts: min_wait_time,
                            x: n_x,
                            y: n_y,
                        });
                    }
                }
            }
        }
        time[m - 1][n - 1] as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![56,93],vec![3,38]], 39),
            (
                vec![vec![94, 79, 62, 27, 69, 84], vec![6, 32, 11, 82, 42, 30]],
                72,
            ),
            (vec![vec![0, 4], vec![4, 4]], 6),
            (vec![vec![0, 0, 0], vec![0, 0, 0]], 3),
            (vec![vec![0, 1], vec![1, 2]], 3),
            (vec![vec![15, 58], vec![67, 4]], 60),
            (vec![vec![17, 56], vec![97, 80]], 81),
            (
                vec![
                    vec![63, 102, 19, 11, 110, 26, 89, 101, 19],
                    vec![7, 17, 119, 94, 44, 3, 30, 111, 54],
                    vec![20, 66, 72, 19, 76, 6, 16, 0, 97],
                    vec![7, 48, 113, 22, 14, 100, 99, 91, 60],
                ],
                93,
            ),
        ]
    }

    #[test]
    fn test_min_time_to_reach() {
        for (move_time, want) in testcases() {
            assert_eq!(Solution::min_time_to_reach(move_time), want);
        }
    }
}
