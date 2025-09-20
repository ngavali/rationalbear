//https://leetcode.com/problems/spiral-matrix/
struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut factor = 0;
        let size = matrix.len() * matrix[0].len();
        let mut ans = Vec::new();
        let (mut i, mut j) = (0, 0);
        while ans.len() < size {
            while j < matrix[0].len() - factor {
                ans.push(matrix[i][j]);
                j += 1;
            }
            j -= 1;
            i += 1;
            while i < matrix.len() - factor {
                ans.push(matrix[i][j]);
                i += 1;
            }
            i -= 1;
            while j > factor {
                j -= 1;
                ans.push(matrix[i][j]);
            }
            factor += 1;
            while i > factor {
                i -= 1;
                ans.push(matrix[i][j]);
            }
            i = factor;
            j = factor;
        }
        //Do this or check ans len in each of the inner while loop
        while ans.len() > size {
            ans.pop();
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(Vec<Vec<i32>>, Vec<i32>)> {
        vec![
            (vec![vec![1, 2, 3, 4]], vec![1, 2, 3, 4]),
            (
                vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]],
                vec![1, 2, 3, 4, 8, 7, 6, 5],
            ),
            (
                vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                    vec![10, 11, 12],
                    vec![13, 14, 15],
                ],
                vec![1, 2, 3, 6, 9, 12, 15, 14, 13, 10, 7, 4, 5, 8, 11],
            ),
            (vec![vec![1]], vec![1]),
            (
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16],
                ],
                vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
            ),
            (
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![6, 7, 8, 9, 10],
                    vec![11, 12, 13, 14, 15],
                ],
                vec![1, 2, 3, 4, 5, 10, 15, 14, 13, 12, 11, 6, 7, 8, 9],
            ),
        ]
    }

    #[test]
    fn test_spiral_order() {
        for (matrix, want) in testcases() {
            assert_eq!(Solution::spiral_order(matrix), want);
        }
    }
}
