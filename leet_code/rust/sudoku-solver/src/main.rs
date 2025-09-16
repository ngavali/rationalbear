//https://leetcode.com/problems/sudoku-solver/
struct Solution;

impl Solution {

    fn get_next_empty_cell(
        size: usize,
        mut pos: (usize, usize),
        grid_numeric: &[Vec<usize>],
    ) -> Option<(usize, usize)> {
        while pos.0 < size {
            while pos.1 < size {
                if grid_numeric[pos.0][pos.1] == 0 {
                    return Some(pos);
                }
                pos.1 += 1
            }
            pos.0 += 1;
            pos.1 = 0;
        }
        None
    }

    fn backtrack(
        size: usize,
        pos: &(usize, usize),
        grid_numeric: &mut [Vec<usize>],
        row_clue_flags: &mut [i32],
        col_clue_flags: &mut [i32],
        three_by_three_boxes_flags: &mut [i32],
        solution: &mut bool,
    ) {

        if let Some(pos) = Self::get_next_empty_cell(size, *pos, grid_numeric) {
            for num in 1..=9 {
                let mask = 1 << num;
                if (row_clue_flags[pos.0] & mask) == 0
                    && (col_clue_flags[pos.1] & mask) == 0
                    && (three_by_three_boxes_flags[3 * (pos.0 / 3) + pos.1 / 3] & mask) == 0
                 {
                    grid_numeric[pos.0][pos.1] = num;
                    row_clue_flags[pos.0] |= mask;
                    col_clue_flags[pos.1] |= mask;
                    three_by_three_boxes_flags[3 * (pos.0 / 3) + pos.1 / 3] |= mask;
                    Self::backtrack(
                        size,
                        &pos,
                        grid_numeric,
                        row_clue_flags,
                        col_clue_flags,
                        three_by_three_boxes_flags,
                        solution,
                    );
                    if !*solution {
                        grid_numeric[pos.0][pos.1] = 0;
                        row_clue_flags[pos.0] ^= mask;
                        col_clue_flags[pos.1] ^= mask;
                        three_by_three_boxes_flags[3 * (pos.0 / 3) + pos.1 / 3] ^= mask;
                    }
                }
            }
        } else {
            //There are no more empty places to fill
            //We have a solution
            *solution = true;
        }
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let mut grid_numeric = vec![vec![0; n]; n];
        let mut row_clue_flags = vec![0; n];
        let mut col_clue_flags = vec![0; n];
        let mut three_by_three_boxes_flags = vec![0; n];
        board.into_iter().enumerate().for_each(|(i, row) |{
            row.into_iter().enumerate().for_each(|(j, num) |{
                grid_numeric[i][j] = num.to_digit(10).unwrap_or(0) as usize;
                if grid_numeric[i][j] != 0 {
                    let mask = 1 << grid_numeric[i][j];
                    row_clue_flags[i] |= mask;
                    col_clue_flags[j] |= mask;
                    three_by_three_boxes_flags[3 * (i / 3) + j / 3] |= mask;
                }
            })
        });
        let mut solution = false;

        Self::backtrack(
            n,
            &(0, 0),
            &mut grid_numeric,
            &mut row_clue_flags,
            &mut col_clue_flags,
            &mut three_by_three_boxes_flags,
            &mut solution,
        );

        grid_numeric.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter()
                .enumerate()
                .for_each(|(j, col)| board[i][j] = char::from_digit(col as u32, 10).unwrap())
        });
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;
    fn testcases() -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
        vec![(
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ],
        )]
    }

    #[test]
    fn test_solve_sudoku() {
        for (mut board, want) in testcases() {
            Solution::solve_sudoku(&mut board);
            assert_eq!(board, want);
        }
    }
}
