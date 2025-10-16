//https://leetcode.com/problems/generate-parentheses/
struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut valid_parenthesis = Vec::new();

        let mut text = String::new();

        Self::generate_parenthesis_internal(
            &mut valid_parenthesis,
            &mut text,
            0,
            (n * 2) as u8,
            0,
            0,
        );

        valid_parenthesis
    }

    fn generate_parenthesis_internal(
        valid_parenthesis: &mut Vec<String>,
        text: &mut String,
        depth: u8,
        max_depth: u8,
        num_open_parenthesis: u8,
        num_closed_parenthesis: u8,
    ) {
        if depth == max_depth {
            valid_parenthesis.push(text.clone());

            text.pop();

            return;
        }

        if num_open_parenthesis == num_closed_parenthesis {
            text.push('(');

            Self::generate_parenthesis_internal(
                valid_parenthesis,
                text,
                depth + 1,
                max_depth,
                num_open_parenthesis + 1,
                num_closed_parenthesis,
            );
        } else if max_depth - num_open_parenthesis == num_open_parenthesis {
            text.push(')');

            Self::generate_parenthesis_internal(
                valid_parenthesis,
                text,
                depth + 1,
                max_depth,
                num_open_parenthesis,
                num_closed_parenthesis + 1,
            );
        } else {
            text.push('(');

            Self::generate_parenthesis_internal(
                valid_parenthesis,
                text,
                depth + 1,
                max_depth,
                num_open_parenthesis + 1,
                num_closed_parenthesis,
            );

            text.push(')');

            Self::generate_parenthesis_internal(
                valid_parenthesis,
                text,
                depth + 1,
                max_depth,
                num_open_parenthesis,
                num_closed_parenthesis + 1,
            );
        }

        text.pop();
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, Vec<String>)> {
        vec![
            (
                3,
                vec![
                    "((()))".to_string(),
                    "(()())".to_string(),
                    "(())()".to_string(),
                    "()(())".to_string(),
                    "()()()".to_string(),
                ],
            ),
            (1, vec!["()".to_string()]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_generate_parenthesis() {
        for (n, want) in testcases() {
            assert_eq!(Solution::generate_parenthesis(n), want);
        }
    }
}
