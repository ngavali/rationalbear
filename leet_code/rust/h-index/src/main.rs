//https://leetcode.com/problems/h-index

struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a,b| b.cmp(&a) );
        for i in 0..citations.len() {
            let a = i as i32;
            if citations[i] <= a {
                return a;
            }
        }
        citations.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![3, 0, 6, 1, 5], 3),
            (vec![1, 3, 1], 1),
            (vec![1], 1),
            (vec![1, 1, 1, 1], 1),
            (vec![7, 7, 7, 7], 4),
            (vec![9, 9, 8, 8, 7, 7, 6, 6, 5], 6),
            (vec![1, 3, 1], 1),
            (
                vec![
                    0, 0, 0, 0, 0, 9, 9, 9, 7, 7, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 9,
                    9, 9, 7, 7, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                7,
            ),
            (vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 3, 5, 5, 2, 5], 4),
            (vec![3, 0, 6, 1, 5], 3),
            (vec![3, 0, 6, 1, 5, 3, 0, 6, 1, 5], 4),
            (vec![7, 7, 7, 8, 8], 5),
            (vec![1, 1, 1, 1, 2, 2, 2, 3, 3], 2),
        ]
    }

    use super::Solution;
    #[test]
    fn test_h_index() {
        for (citations, want) in testcases() {
            assert_eq!(Solution::h_index(citations), want);
        }
    }
}
