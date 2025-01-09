/* Medium
 * https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
 */

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] - prices[i-1] > 0 {
                profit += prices[i] - prices[i-1];
            }
        }
        profit
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::iter::Inspect;

    use super::Solution;

    #[test]
    fn test_max_profit() {
        let mut test_cases = vec![
            (vec![7, 1, 5, 3, 6, 4], 7),
            (vec![1, 2, 3, 4, 5], 4),
            (vec![7, 6, 4, 3, 1], 0),
        ];

        for (input, output) in test_cases.into_iter() {
            assert_eq!(output, Solution::max_profit(input));
        }
    }
}
