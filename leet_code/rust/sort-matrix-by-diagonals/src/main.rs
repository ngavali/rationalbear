//https://leetcode.com/problems/sort-matrix-by-diagonals/
use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        for i in 0..n {
            let mut list = Vec::<Reverse<i32>>::new();
            for j in 0..(n - i) {
                //println!("{},{} -> {}", i + j, j, grid[i + j][j]);
                list.push(Reverse(grid[i + j][j]));
            }
            //println!("{:?}", list);
            list.sort();
            //println!("Sorted = {:?}", list);
            for j in 0..(n - i) {
                grid[i + j][j] = list[j].0;
            }
        }
        for j in 1..n {
            let mut list = Vec::<i32>::new();
            for i in 0..(n - j) {
                println!("{},{} -> {}", i + j, j + 1, grid[i][i + j]);
                list.push(grid[i][i + j]);
            }
            println!("{:?}", list);
            list.sort();
            println!("Sorted = {:?}", list);
            for i in 0..(n - j) {
                grid[i][i + j] = list[i];
            }
        }

        grid
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    #[test]
    fn test_sort_matrix() {
        let testcases = vec![
            (
                vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]],
                vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]],
            ),
            (vec![vec![0, 1], vec![1, 2]], vec![vec![2, 1], vec![1, 0]]),
            (vec![vec![1]], vec![vec![1]]),
        ];

        for testcase in testcases {
            assert_eq!(testcase.1, Solution::sort_matrix(testcase.0));
        }
    }
}
