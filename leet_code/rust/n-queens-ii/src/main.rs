//https://leetcode.com/problems/n-queens-ii/
struct Solution {}

impl Solution {
    fn can_place(chessboard: &mut Vec<usize>, pos: (usize, usize)) -> bool {
        for (row, &col) in chessboard.iter().enumerate() {
            //No need to check same row, we never do that!
            if pos.1 == col || (row.abs_diff(pos.0) == col.abs_diff(pos.1)) {
                return false;
            }
        }

        true
    }

    fn solve(bs: usize, row: usize, chessboard: &mut Vec<usize>) -> i32{
        let mut ans= 0;
        if row == bs {
            return 1;
        }
        for j in 0..bs {
            if Self::can_place(chessboard, (row, j)) {
                chessboard.push(j);
                ans +=Self::solve(bs, row + 1, chessboard);
                chessboard.pop();
            }
        }
        ans
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut chessboard = Vec::<usize>::with_capacity(n as usize);
        Self::solve(n as usize, 0, &mut chessboard)
    }
}

fn main() {
    println!("vec![");

    for i in 1..10 {
        print!("{},", Solution::total_n_queens(i));
    }
    println!("]") ;
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(i32, i32)> {
        vec![(4, 2), (1, 1)]
    }

    #[test]
    fn test_total_n_queens() {
        for testcase in testcases().into_iter() {
            assert_eq!(Solution::total_n_queens(testcase.0), testcase.1);
        }
    }
}
