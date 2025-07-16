use std::usize;

/* Medium
 * https://leetcode.com/problems/surrounded-regions/
 */
struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let M = board.len();
        let N = board[0].len();

        fn dfs(board: &mut Vec<Vec<char>>, idx: (usize, usize), range: (usize, usize)) {
            if board[idx.0][idx.1] == 'X' || board[idx.0][idx.1] == 'V' {
                return;
            }
            board[idx.0][idx.1] = 'V';
            if idx.1 > 0 {
                dfs(board, (idx.0, idx.1 - 1), range);
            }
            if idx.1 + 1 < range.1 {
                dfs(board, (idx.0, idx.1 + 1), range);
            }
            if idx.0 + 1 < range.0 {
                dfs(board, (idx.0 + 1, idx.1), range);
            }
            if idx.0 > 0 {
                dfs(board, (idx.0 - 1, idx.1), range);
            }
        }
        for i in { 0..M } {
            for j in { 0..N } {
                if (i == 0 || i == M - 1) || (j == 0 || j == N - 1) {
                    if board[i][j] == 'O' {
                        dfs(board, (i, j), (M, N));
                    }
                }
            }
        }
        for i in { 0..M } {
            for j in { 0..N } {
                if board[i][j] == 'V' {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve() {
        let test_cases: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = vec![
            (
                vec![
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'O', 'O', 'X'],
                    vec!['X', 'X', 'O', 'X'],
                    vec!['X', 'O', 'X', 'X'],
                ],
                vec![
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'X', 'X', 'X'],
                    vec!['X', 'O', 'X', 'X'],
                ],
            ),
            (vec![vec!['X']], vec![vec!['X']]),
            (
                vec![
                    vec!['O', 'O', 'O', 'O', 'X', 'X'],
                    vec!['O', 'O', 'O', 'O', 'O', 'O'],
                    vec!['O', 'X', 'O', 'X', 'O', 'O'],
                    vec!['O', 'X', 'O', 'O', 'X', 'O'],
                    vec!['O', 'X', 'O', 'X', 'O', 'O'],
                    vec!['O', 'X', 'O', 'O', 'O', 'O'],
                ],
                vec![
                    vec!['O', 'O', 'O', 'O', 'X', 'X'],
                    vec!['O', 'O', 'O', 'O', 'O', 'O'],
                    vec!['O', 'X', 'O', 'X', 'O', 'O'],
                    vec!['O', 'X', 'O', 'O', 'X', 'O'],
                    vec!['O', 'X', 'O', 'X', 'O', 'O'],
                    vec!['O', 'X', 'O', 'O', 'O', 'O'],
                ],
            ),
            (
                vec![
                    vec!['X', 'O', 'X'],
                    vec!['O', 'X', 'O'],
                    vec!['X', 'O', 'X'],
                ],
                vec![
                    vec!['X', 'O', 'X'],
                    vec!['O', 'X', 'O'],
                    vec!['X', 'O', 'X'],
                ],
            ),
        ];

        for (i, (mut board, expected)) in test_cases.into_iter().enumerate() {
            println!("Testcase #{i}");
            Solution::solve(&mut board);
            assert_eq!(expected, board);
        }
    }
}
