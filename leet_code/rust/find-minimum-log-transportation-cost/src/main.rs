//https://leetcode.com/problems/find-minimum-log-transportation-cost/
struct Solution;

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        (if n > k { k as i64 * (n - k) as i64 } else { 0 })
            + (if m > k { k as i64 * (m - k) as i64 } else { 0 })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, i32, i32, i64)> {
        vec![
            (1, 1, 100, 0),
            (100, 100, 100, 0),
            (6, 5, 5, 5),
            (5, 6, 5, 5),
            (5, 4, 4, 4),
            (5, 5, 5, 0),
            (10, 4, 5, 25),
            (7, 6, 7, 0),
        ]
    }

    use super::Solution;
    #[test]
    fn test_min_cutting_cost() {
        for (n, m, k, want) in testcases() {
            assert_eq!(Solution::min_cutting_cost(n, m, k), want);
        }
    }
}
