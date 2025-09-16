//https://leetcode.com/problems/sudoku-solver/
struct Solution;

impl Solution {
    fn get_box_number(pos: (usize, usize)) -> usize {
        3 * (pos.0 / 3) + pos.1 / 3
    }

    fn is_number_used(
        number: usize,
        pos: (usize, usize),
        row_clue: &mut Vec<Vec<bool>>,
        col_clue: &mut Vec<Vec<bool>>,
        three_by_three_boxes: &mut Vec<Vec<bool>>,
    ) -> bool {
        row_clue[pos.0][number]
            || col_clue[pos.1][number]
            || three_by_three_boxes[Self::get_box_number(pos)][number]
    }

    fn write_number(
        number: usize,
        pos: (usize, usize),
        grid_numberic: &mut Vec<Vec<usize>>,
        row_clue: &mut Vec<Vec<bool>>,
        col_clue: &mut Vec<Vec<bool>>,
        three_by_three_boxes: &mut Vec<Vec<bool>>,
    ) {
        grid_numberic[pos.0][pos.1] = number;
        row_clue[pos.0][number] = true;
        col_clue[pos.1][number] = true;
        three_by_three_boxes[Self::get_box_number(pos)][number] = true;
    }

    fn erase_number(
        number: usize,
        pos: (usize, usize),
        grid_numberic: &mut Vec<Vec<usize>>,
        row_clue: &mut Vec<Vec<bool>>,
        col_clue: &mut Vec<Vec<bool>>,
        three_by_three_boxes: &mut Vec<Vec<bool>>,
    ) {
        grid_numberic[pos.0][pos.1] = 0;
        row_clue[pos.0][number] = false;
        col_clue[pos.1][number] = false;
        three_by_three_boxes[Self::get_box_number(pos)][number] = false;
    }

    fn get_next_empty_cell(
        size: usize,
        mut pos: (usize, usize),
        grid_numberic: &Vec<Vec<usize>>,
    ) -> Option<(usize, usize)> {
        while pos.0 < size {
            while pos.1 < size {
                if grid_numberic[pos.0][pos.1] == 0 {
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
        pos: (usize, usize),
        grid_numberic: &mut Vec<Vec<usize>>,
        row_clue: &mut Vec<Vec<bool>>,
        col_clue: &mut Vec<Vec<bool>>,
        three_by_three_boxes: &mut Vec<Vec<bool>>,
        grid_final: &mut Vec<Vec<usize>>,
    ) {
        if let Some(pos) = Self::get_next_empty_cell(size, pos, grid_numberic) {
            for num in 1..=9 {
                if !Self::is_number_used(
                    num,
                    (pos.0, pos.1),
                    row_clue,
                    col_clue,
                    three_by_three_boxes,
                ) {
                    Self::write_number(
                        num,
                        (pos.0, pos.1),
                        grid_numberic,
                        row_clue,
                        col_clue,
                        three_by_three_boxes,
                    );
                    Self::backtrack(
                        size,
                        pos,
                        grid_numberic,
                        row_clue,
                        col_clue,
                        three_by_three_boxes,
                        grid_final,
                    );
                    Self::erase_number(
                        num,
                        (pos.0, pos.1),
                        grid_numberic,
                        row_clue,
                        col_clue,
                        three_by_three_boxes,
                    );
                }
            }
        } else {
            grid_numberic.iter().enumerate().for_each(|(i, row)| {
                row.iter()
                    .enumerate()
                    .for_each(|(j, col)| grid_final[i][j] = *col);
            });
        }
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let mut grid_numberic = vec![vec![0; n]; n];
        let mut row_clue = vec![vec![false; n + 1]; n];
        let mut col_clue = vec![vec![false; n + 1]; n];
        let mut three_by_three_boxes = vec![vec![false; n + 1]; n];
        let mut grid_final = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                grid_numberic[i][j] = board[i][j].to_digit(10).unwrap_or(0) as usize;
                if grid_numberic[i][j] != 0 {
                    row_clue[i][grid_numberic[i][j]] = true;
                    col_clue[j][grid_numberic[i][j]] = true;
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if grid_numberic[i][j] != 0 {
                    three_by_three_boxes[Self::get_box_number((i, j))][grid_numberic[i][j]] = true;
                }
            }
        }
        Self::backtrack(
            n,
            (0, 0),
            &mut grid_numberic,
            &mut row_clue,
            &mut col_clue,
            &mut three_by_three_boxes,
            &mut grid_final,
        );

        grid_final.into_iter().enumerate().for_each(|(i, row)| {
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
