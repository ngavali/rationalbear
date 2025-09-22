struct Solution;

use std::collections::BinaryHeap;

//Use struct with custom cmp impl so that in sort only necessary comparisons are done
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Brick {
    height: i32,
    pos: (usize, usize),
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height)
    }
}

impl Solution {
    pub fn trap_rain_water(n_map: Vec<Vec<i32>>) -> i32 {
        let m = n_map.len() as usize;
        let n = n_map[0].len() as usize;
        let mut bricks: BinaryHeap<Brick> = BinaryHeap::new();
        let mut visited_bricks = vec![vec![false; n]; m];
        n_map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &height)| {
                if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                    visited_bricks[i][j] = true;
                    bricks.push(Brick {
                        height: height,
                        pos: (i, j),
                    });
                }
            })
        });
        let mut trapped_water_quantity = 0;
        while let Some(Brick { height, pos }) = bricks.pop() {
            let next_positions = vec![
                (pos.0, pos.1.wrapping_sub(1)),
                (pos.0, pos.1 + 1),
                (pos.0.wrapping_sub(1), pos.1),
                (pos.0 + 1, pos.1),
            ];
            for (r, c) in next_positions {
                if r < m && c < n && !visited_bricks[r][c] {
                    let curr_height = n_map[r][c];
                    trapped_water_quantity += 0.max(height - curr_height);
                    bricks.push(Brick {
                        height: height.max(curr_height),
                        pos: (r, c),
                    });
                    visited_bricks[r][c] = true;
                }
            }
        }
        trapped_water_quantity
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (
                vec![
                    vec![1, 4, 3, 1, 3, 2],
                    vec![3, 2, 1, 3, 2, 4],
                    vec![2, 3, 3, 2, 3, 1],
                ],
                4,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 2, 1, 2, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 3, 3, 3, 3],
                ],
                10,
            ),
            (
                vec![
                    vec![12, 13, 1, 12],
                    vec![13, 4, 13, 12],
                    vec![13, 8, 10, 12],
                    vec![12, 13, 12, 12],
                    vec![13, 13, 13, 13],
                ],
                14,
            ),
            (
                vec![
                    vec![2, 3, 4],
                    vec![5, 6, 7],
                    vec![8, 9, 10],
                    vec![11, 12, 13],
                    vec![14, 15, 16],
                ],
                0,
            ),
            (
                vec![
                    vec![9, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9],
                    vec![9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 9],
                    vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                    vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                    vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
                ],
                215,
            ),
        ]
    }

    #[test]
    fn test_trap_rain_water() {
        for (n_map, want) in testcases() {
            assert_eq!(Solution::trap_rain_water(n_map), want);
        }
    }
}
