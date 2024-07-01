//Previous approach was incorrect,
//We need to find max height as we drop squares not from left to right
//Let's do Naive implementation first
const DEBUG: bool = true;

struct StackHeight;

impl StackHeight {
    fn from(squares: &mut Vec<Vec<i32>>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(squares.len());
        let mut height_memory = Vec::with_capacity(squares.len());
        let mut max_height = 0;

        for i in 0..squares.len() {
            let mut current_height = 0;
            for j in 0..i {
                if DEBUG {
                    println!(
                        "i->{i} {:?} range [{} {}]",
                        squares[i],
                        squares[i][0],
                        squares[i][0] + squares[i][1]
                    );
                }
                //Check if this square sits on top of the previous one
                if squares[i][0] + squares[i][1] > squares[j][0]
                    && squares[i][0] < squares[j][0] + squares[j][1]
                {
                    if DEBUG {
                        println!(
                            " j -> {j} {:?} range [{} {}]",
                            squares[j],
                            squares[j][0],
                            squares[j][0] + squares[j][1]
                        );
                    }
                    if current_height < height_memory[j] {
                        current_height = height_memory[j];
                    }
                }
            }
            if max_height < current_height + squares[i][1] {
                max_height = current_height + squares[i][1]
            }
            height_memory.push(current_height + squares[i][1]);
            if DEBUG {
                println!(
                    "max height -> {} current_height -> {} height_memory -> {:?} stack -> {:?}",
                    max_height,
                    current_height + squares[i][1],
                    height_memory,
                    stack
                );
            }
            stack.push(max_height);
        }

        stack
    }
}

struct Solution;

impl Solution {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        StackHeight::from(&mut positions)
    }
}

fn main() {
    let test_cases = vec![(vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8])];

    for test_case in test_cases {
        println!("{:#?}", Solution::falling_squares(test_case.0));
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_falling_squares() {
        let test_cases = vec![
            (vec![vec![1, 2], vec![2, 3], vec![6, 1]], vec![2, 5, 5]),
            (vec![vec![100, 100], vec![200, 100]], vec![100, 100]),
            (vec![vec![6, 1], vec![9, 2], vec![2, 4]], vec![1, 2, 4]),
            (vec![vec![9, 7], vec![1, 9], vec![3, 1]], vec![7, 16, 17]),
            (vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8]),
        ];

        for test_case in test_cases {
            assert_eq!(test_case.1, Solution::falling_squares(test_case.0));
        }
    }
}
