//https://leetcode.com/problems/n-queens/
struct Solution {}

impl Solution {
    fn fillQueen(pos: (usize, usize), bs: usize, placements: &mut Vec<Vec<u8>>) {
        for j in 0..bs {
            placements[pos.0][j] += 1;
            if j != pos.0 {
                placements[j][pos.1] += 1;
            }
        }
        let mut inc = 1;
        for i in (0..pos.0).take(pos.0).rev() {
            if (pos.1 + inc) < bs {
                placements[i][pos.1 + inc] += 1;
            }
            if pos.1 >= inc {
                placements[i][pos.1 - inc] += 1;
            }
            inc += 1;
        }
        let mut inc = 1;
        for i in (pos.0 + 1)..bs {
            if (pos.1 + inc) < bs {
                placements[i][pos.1 + inc] += 1;
            }
            if pos.1 >= inc {
                placements[i][pos.1 - inc] += 1;
            }
            inc += 1;
        }
    }

    fn removeQueen(pos: (usize, usize), bs: usize, placements: &mut Vec<Vec<u8>>) {
        for j in 0..bs {
            if placements[pos.0][j] > 0 {
                placements[pos.0][j] -= 1;
            }
            if placements[j][pos.1] > 0 && j != pos.0 {
                placements[j][pos.1] -= 1;
            }
        }
        let mut inc = 1;
        for i in (0..pos.0).take(pos.0).rev() {
            if (pos.1 + inc) < bs {
                placements[i][pos.1 + inc] -= 1;
            }
            if pos.1 >= inc {
                placements[i][pos.1 - inc] -= 1;
            }
            inc += 1;
        }
        let mut inc = 1;
        for i in (pos.0 + 1)..bs {
            if (pos.1 + inc) < bs {
                placements[i][pos.1 + inc] -= 1;
            }
            if pos.1 >= inc {
                placements[i][pos.1 - inc] -= 1;
            }
            inc += 1;
        }
    }

    fn solve_me(
        bs: usize,
        pos: (usize, usize),
        board: &mut Vec<Vec<u8>>,
        placements: &mut Vec<Vec<u8>>,
        queens_placed: usize,
        solutions: &mut Vec<Vec<String>>,
    ) {
        if queens_placed == bs {
            let mut solution = Vec::<String>::new();
            for row in board.iter() {
                solution.push(String::from_utf8(row.clone()).unwrap());
            }
            solutions.push(solution);
        }
        if pos.0 < bs {
            for j in 0..bs {
                if placements[pos.0][j] == 0 {
                    board[pos.0][j] = b'Q';
                    Self::fillQueen((pos.0, j), bs, placements);
                    Self::solve_me(
                        bs,
                        (pos.0 + 1, j + 1),
                        board,
                        placements,
                        queens_placed + 1,
                        solutions,
                    );
                    Self::removeQueen((pos.0, j), bs, placements);
                    board[pos.0][j] = b'.';
                }
            }
        }
    }
    pub fn solve_n_queens_me(n: i32) -> Vec<Vec<String>> {
        let mut solutions = Vec::<Vec<String>>::new();
        let mut board = vec![vec![b'.'; n as usize]; n as usize];
        let mut placements: Vec<Vec<u8>> = vec![vec![0; n as usize]; n as usize];
        Self::solve_me(
            n as usize,
            (0, 0),
            &mut board,
            &mut placements,
            0,
            &mut solutions,
        );

        solutions
    }

    //Implementing some of the best solutions that I found interesting, space optimization
    fn can_place(chessboard: &mut Vec<usize>, pos: (usize, usize)) -> bool {
        for (row, &col) in chessboard.iter().enumerate() {
            //No need to check same row, we never do that!
            if pos.1 == col || (row.abs_diff(pos.0) == col.abs_diff(pos.1)) {
                return false;
            }
        }

        true
    }

    fn solve(bs: usize, row: usize, chessboard: &mut Vec<usize>, solutions: &mut Vec<Vec<String>>) {
        if row == bs {
            let mut solution = vec![vec!['.'; bs]; bs];
            for (row, &col) in chessboard.iter().enumerate() {
                solution[row][col] = 'Q';
            }
            solutions.push(
                solution
                    .into_iter()
                    .map(|row| row.into_iter().collect())
                    .collect(),
            );
        }
        for j in 0..bs {
            if Self::can_place(chessboard, (row, j)) {
                chessboard.push(j);
                Self::solve(bs, row + 1, chessboard, solutions);
                chessboard.pop();
            }
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solutions = Vec::<Vec<String>>::new();
        let mut chessboard = Vec::<usize>::with_capacity(n as usize);
        Self::solve(n as usize, 0, &mut chessboard, &mut solutions);

        solutions
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(i32, Vec<Vec<String>>)> {
        vec![
            (
                4,
                vec![
                    vec![
                        String::from(".Q.."),
                        String::from("...Q"),
                        String::from("Q..."),
                        String::from("..Q."),
                    ],
                    vec![
                        String::from("..Q."),
                        String::from("Q..."),
                        String::from("...Q"),
                        String::from(".Q.."),
                    ],
                ],
            ),
            (1, vec![vec![String::from("Q")]]),
        ]
    }

    #[test]
    fn test_solve_n_queens() {
        for testcase in testcases().into_iter() {
            let mut got = Solution::solve_n_queens(testcase.0);
            let mut want = testcase.1;
            got.sort();
            want.sort();
            //println!(";got {:?}\nwant {:?}", got, want);
            assert_eq!(got, want);
        }
    }
    #[test]
    fn test_solve_n_queens_me() {
        for testcase in testcases().into_iter() {
            let mut got = Solution::solve_n_queens_me(testcase.0);
            let mut want = testcase.1;
            got.sort();
            want.sort();
            //println!(";got {:?}\nwant {:?}", got, want);
            assert_eq!(got, want);
        }
    }
}
