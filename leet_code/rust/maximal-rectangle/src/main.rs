struct Solution {}

//Read largest-rectangle-in-histogram before attempting this problem
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights: Vec<i32> = vec![0; matrix[0].len()];
        let mut max_area = 0;
        for curr_height in matrix {
            heights = curr_height
                .iter()
                .enumerate()
                .map(|(i, &val)| if val == '1' { heights[i] + 1 } else { 0 })
                .collect();
            max_area = max_area.max({
                    let mut stack = Vec::with_capacity(heights.len());
                    let n = heights.len();
                    for i in 0..n {
                        while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
                            let top = stack.pop().unwrap();
                            max_area = max_area.max(if !stack.is_empty() {
                                heights[top] * (i - *stack.last().unwrap() - 1) as i32
                            } else {
                                heights[top] * i as i32
                            });
                        }
                        stack.push(i);
                    }
                    while !stack.is_empty() {
                        let top = stack.pop().unwrap();
                        max_area = max_area.max(if !stack.is_empty() {
                            heights[top] * (n - *stack.last().unwrap() - 1) as i32
                        } else {
                            heights[top] * n as i32
                        });
                    }
                    max_area
                });
        }
        max_area
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<char>>, i32)> {
        vec![
            (
                vec![
                    vec!['1', '0', '1', '0', '0'],
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '0', '1', '0'],
                ],
                6,
            ),
            (vec![vec!['0']], 0),
            (vec![vec!['1']], 1),
        ]
    }

    #[test]
    fn test_maximal_rectangle() {
        for (i, (heights, want)) in testcases().into_iter().enumerate() {
            println!("---- Testcase #{i} ----");
            assert_eq!(Solution::maximal_rectangle(heights), want);
        }
    }
}
