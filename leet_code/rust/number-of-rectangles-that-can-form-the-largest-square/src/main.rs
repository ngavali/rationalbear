//https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/

struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_squares = (0, 0);
        rectangles.iter().for_each(|a| {
            let s = a[0].min(a[1]);
            if s > max_squares.0 {
                max_squares.0 = s;
                max_squares.1 = 1;
            } else if s == max_squares.0 {
                max_squares.1 += 1;
            }
        });
        max_squares.1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]], 3),
            (vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]], 3),
        ]
    }

    use super::Solution;
    #[test]
    fn test_count_good_rectangles() {
        for (rectangles, want) in testcases() {
            assert_eq!(Solution::count_good_rectangles(rectangles), want);
        }
    }
}
