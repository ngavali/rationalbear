//https://leetcode.com/problems/find-the-number-of-winning-players/
struct Solution;
struct SolutionHashMap;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut collection: Vec<Vec<i32>> = vec![vec![0; 11]; n as usize];
        for p in pick {
            collection[p[0] as usize][p[1] as usize] += 1;
        }
        let mut ans = 0;
        for (p, b_freq) in collection.iter().enumerate() {
            if let Some(&f) = b_freq.iter().max()
                && (p as i32) < f
            {
                ans += 1;
            }
        }
        ans
    }
}

use std::collections::HashMap;
impl SolutionHashMap {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut collection: Vec<Vec<i32>> = vec![vec![0; 11]; n as usize];
        for p in pick {
            collection[p[0] as usize][p[1] as usize] += 1;
        }
        let mut ans = 0;
        for (p, b_freq) in collection.iter().enumerate() {
            //println!("{p} {b_freq:?}");
            if let Some(&f) = b_freq.iter().max()
                && (p as i32) < f
            {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, Vec<Vec<i32>>, i32)> {
        vec![
            (
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0],
                ],
                2,
            ),
            (5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]], 0),
            (5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]], 1),
        ]
    }
    use super::SolutionHashMap;
    #[test]
    fn test_winning_player_count_hashmap() {
        for (n, pick, want) in testcases() {
            assert_eq!(SolutionHashMap::winning_player_count(n, pick), want);
        }
    }

    use super::Solution;
    #[test]
    fn test_winning_player_count() {
        for (n, pick, want) in testcases() {
            assert_eq!(Solution::winning_player_count(n, pick), want);
        }
    }
}
