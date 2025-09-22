//https://leetcode.com/problems/spiral-matrix-ii/
struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut factor = 0;
        let size = n*n;
        let n = n as usize;
        let mut matrix = vec![vec![0;n];n];
        let (mut i, mut j) = (0, 0);
        let mut ans: i32 = 0;
        while ans < size {
            let c = n - factor;
            while j < c {
                matrix[i][j] = ans+1;
                j += 1;
                ans+=1;
            }
            if ans > size {break;} 
            j -= 1;
            i += 1;
            while i < c {
                matrix[i][j] = ans+1;
                i += 1;
                ans+=1;
            }
            if ans > size {break;} 
            i -= 1;
            while j > factor {
                j -= 1;
                matrix[i][j] = ans+1;
                ans+=1;
            }
            if ans > size {break;} 
            factor += 1;
            while i > factor {
                i -= 1;
                matrix[i][j] = ans+1;
                ans+=1;
            }
            i = factor;
            j = factor;
        }
        matrix
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(i32, Vec<Vec<i32>>)> {
        vec![
            (3,
                vec![
                    vec![1, 2, 3],
                    vec![8, 9, 4],
                    vec![7, 6, 5],
                ],
            ),
        ]
    }

    #[test]
    fn test_spiral_order() {
        for (n, want) in testcases() {
            assert_eq!(Solution::generate_matrix(n), want);
        }
    }
}
