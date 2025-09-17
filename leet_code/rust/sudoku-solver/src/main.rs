//https://leetcode.com/problems/sudoku-solver/
struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn backtrack(
        size: usize,
        pos: (usize, usize),
        grid_numeric: &mut [Vec<usize>],
        row_clue_flags: &mut [i32],
        col_clue_flags: &mut [i32],
        three_by_three_boxes_flags: &mut [i32],
        solution: &mut bool,
        empty_cells: &mut VecDeque<((usize, usize), Vec<usize>)>,
    ) {
        if let Some((pos, choices)) = empty_cells.pop_front() {
            for &num in choices.iter() {
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
                        pos,
                        grid_numeric,
                        row_clue_flags,
                        col_clue_flags,
                        three_by_three_boxes_flags,
                        solution,
                        empty_cells,
                    );
                    if !*solution {
                        grid_numeric[pos.0][pos.1] = 0;
                        row_clue_flags[pos.0] ^= mask;
                        col_clue_flags[pos.1] ^= mask;
                        three_by_three_boxes_flags[3 * (pos.0 / 3) + pos.1 / 3] ^= mask;
                    }
                }
            }
            if !*solution {
                empty_cells.push_front((pos, choices));
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
        board.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter().enumerate().for_each(|(j, num)| {
                grid_numeric[i][j] = num.to_digit(10).unwrap_or(0) as usize;
                if grid_numeric[i][j] != 0 {
                    let mask = 1 << grid_numeric[i][j];
                    row_clue_flags[i] |= mask;
                    col_clue_flags[j] |= mask;
                    three_by_three_boxes_flags[3 * (i / 3) + j / 3] |= mask;
                }
            })
        });
        let mut empty_cells = Vec::new();
        grid_numeric.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, num)| {
                if grid_numeric[i][j] == 0 {
                    let ones = (row_clue_flags[i] as u32).count_ones()
                        + (col_clue_flags[j] as u32).count_ones()
                        + (three_by_three_boxes_flags[3 * (i / 3) + j / 3] as i32).count_ones();
                    //Lets keep choices prepared
                    let mut possible_choices = Vec::new();
                    for num in 1..=9 {
                        let mask: i32 = 1 << num;
                        if (row_clue_flags[i] & mask) == 0
                            && (col_clue_flags[j] & mask) == 0
                            && (three_by_three_boxes_flags[3 * (i / 3) + j / 3] & mask) == 0
                        {
                            possible_choices.push(num as usize);
                        }
                    }
                    empty_cells.push((ones, (i, j), possible_choices));
                }
            })
        });
        //Lets pick up cells with least choices to fill in
        empty_cells.sort_by(|a, b| a.2.len().cmp(&b.2.len()));
        for row in empty_cells.iter() {
            println!("{row:?} ");
        }
        let mut empty_cells: VecDeque<((usize, usize), Vec<usize>)> =
            empty_cells.into_iter().map(|(_, b, c)| (b, c)).collect();
        let mut solution = false;

        Self::backtrack(
            n,
            (0, 0),
            &mut grid_numeric,
            &mut row_clue_flags,
            &mut col_clue_flags,
            &mut three_by_three_boxes_flags,
            &mut solution,
            &mut empty_cells,
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

/*
 * //Alternate Solution Study

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0b1111111110u16; 9];
        let mut cols = [0b1111111110u16; 9];
        let mut boxes = [0b1111111110u16; 9];
        let mut indexes = board.iter().flatten().enumerate().filter_map(|(idx, &c)| {
            if let Some(digit) = c.to_digit(10) {
                let bit = 1 << digit as u16;
                let mask = !bit;
                let (i, j) = (idx / 9, idx % 9);
                let k = i - i % 3 + j / 3;
                rows[i] &= mask;
                cols[j] &= mask;
                boxes[k] &= mask;
                None
            } else {
                Some(idx)
            }
        }).collect::<Vec<_>>();
        let mut stack = vec![0u16; indexes.len()];
        let mut depth = 0;
        while depth < indexes.len() {
            let old = stack[depth];
            let (new, i, j, k) = if old > 0 {
                let idx = indexes[depth];
                let (i, j) = (idx / 9, idx % 9);
                let k = i - i % 3 + j / 3;
                let bit = old & old.wrapping_neg();
                rows[i] |= bit;
                cols[j] |= bit;
                boxes[k] |= bit;
                (old ^ bit, i, j, k)
            } else {
                let new_depth = indexes.iter().enumerate().skip(depth).min_by_key(|(_, idx)| {
                    let (i, j) = (*idx / 9, *idx % 9);
                    let k = i - i % 3 + j / 3;
                    let bits = rows[i] & cols[j] & boxes[k];
                    bits.count_ones()
                }).unwrap().0;
                indexes.swap(depth, new_depth);
                let idx = indexes[depth];
                let (i, j) = (idx / 9, idx % 9);
                let k = i - i % 3 + j / 3;
                (rows[i] & cols[j] & boxes[k], i, j, k)
            };
            stack[depth] = new;
            if new > 0 {
                let bit = new & new.wrapping_neg();
                let mask = !bit;
                rows[i] &= mask;
                cols[j] &= mask;
                boxes[k] &= mask;
                depth += 1;
            } else {
                depth -= 1;
            }
        }
        for (idx, bits) in indexes.into_iter().zip(stack) {
            let digit = bits.trailing_zeros();
            let (i, j) = (idx / 9, idx % 9);
            board[i][j] = std::char::from_digit(digit, 10).unwrap();
        }
    }
}
*/

/*
 * conversion int to char
 * store corresponding char at given index numeric
 * vec['0','1' ...]
 */
