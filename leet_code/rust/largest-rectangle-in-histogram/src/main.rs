struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(heights.len());
        let mut max_area = 0;
        let n = heights.len();
        for i in 0..n {
            while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
                let top = stack.pop().unwrap();
                max_area = max_area.max(if !stack.is_empty() {
                    heights[top] * (i - *stack.last().unwrap() - 1) as i32
                } else {
                    heights[top] * i as i32
                });
            }
            stack.push(i);
        }
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            max_area = max_area.max(if !stack.is_empty() {
                heights[top] * (n - *stack.last().unwrap() - 1) as i32
            } else {
                heights[top] * n as i32
            });
        }

        max_area
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![(vec![2, 1, 5, 6, 2, 3], 10), (vec![2, 4], 4), (vec![0,9], 9)]
    }

    #[test]
    fn test_calculate_minimum_hp() {
        for (i, (heights, want)) in testcases().into_iter().enumerate() {
            println!("---- Testcase #{i} ----");
            assert_eq!(Solution::largest_rectangle_area(heights), want);
        }
    }
}
