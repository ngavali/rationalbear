//https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cell {
    value: i32,
    x: usize,
    y: usize,
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut sorted_queries = queries.clone();
        sorted_queries.sort();
        let mut query_points: HashMap<i32, i32> = HashMap::new();
        let mut nodes: BinaryHeap<Cell> = BinaryHeap::new();
        let mut total_points = 0;
        nodes.push(Cell {
            value: grid[0][0],
            x: 0,
            y: 0,
        });
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;
        for q in sorted_queries {
            while let Some(&Cell { value, .. }) = nodes.peek()
                && value < q
            {
                if let Some(Cell { value, x, y }) = nodes.pop() {
                    total_points += 1;
                    let direction = [[x, y + 1], [x, y - 1], [x - 1, y], [x + 1, y]];
                    for [x, y] in direction {
                        if x < m && y < n && !visited[x][y] {
                            visited[x][y] = true;
                            nodes.push(Cell {
                                value: grid[x][y],
                                x: x,
                                y: y,
                            });
                        }
                    }
                }
            }
            query_points.insert(q, total_points);
        }
        let result: Vec<i32> = queries
            .into_iter()
            .map(|q| *query_points.get(&q).unwrap_or(&0))
            .collect();
        result
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(Vec<Vec<i32>>, Vec<i32>, Vec<i32>)> {
        vec![
            (
                vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
                vec![5, 6, 2],
                vec![5, 8, 1],
            ),
            (vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3], vec![0]),
            (
                vec![
                    vec![420766, 806051, 922751],
                    vec![181527, 815280, 904568],
                    vec![952102, 4037, 140319],
                    vec![324081, 17907, 799523],
                    vec![176688, 90257, 83661],
                    vec![932477, 621193, 623068],
                    vec![135839, 554701, 511427],
                    vec![227575, 450848, 178065],
                    vec![785644, 204668, 835141],
                    vec![313774, 167359, 501496],
                    vec![641317, 620688, 74989],
                    vec![324499, 122376, 270369],
                    vec![2121, 887154, 848859],
                    vec![456704, 7763, 662087],
                    vec![286827, 145349, 468865],
                    vec![277137, 858176, 725551],
                    vec![106131, 93684, 576512],
                    vec![372563, 944355, 497187],
                    vec![884187, 600892, 268120],
                    vec![576578, 515031, 807686],
                ],
                vec![
                    352655, 586228, 169685, 541073, 584647, 413832, 576537, 616413,
                ],
                vec![0, 2, 0, 2, 2, 0, 2, 2],
            ),
        ]
    }

    #[test]
    fn test_max_points() {
        for (grid, queries, want) in testcases() {
            assert_eq!(Solution::max_points(grid, queries), want);
        }
    }
}
