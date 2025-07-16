/* Easy
 * https://leetcode.com/problems/detect-capital/
 *
 */

struct Solution;

impl Solution {
    fn is_upper_case(w: &[u8]) -> bool {
        for &x in w {
            if !(x > 64 && x < 91) {
                return false;
            }
        }
        true
    }

    fn is_lower_case(w: &[u8]) -> bool {
        for &x in w {
            if !(x > 96 && x < 123) {
                return false;
            }
        }
        true
    }

    pub fn detect_capital_use(word: String) -> bool {
        if word.len() > 1 && word.chars().nth(1) > Some(96 as char) {
            word[1..].chars().all(|x| x > 96 as char)
        } else if word.len() > 1 {
            word.chars().all(|x| x < 91 as char)
        } else {
            true
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn testcases() -> Vec<(String, bool)> {
    vec![(String::from("USA"), true), (String::from("FlaG"), false)]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_detect_capital_use() {
        for (n, (i, o)) in testcases().into_iter().enumerate() {
            println!("Testcase #{n}");
            assert_eq!(o, Solution::detect_capital_use(i));
        }
    }
}
