//https://leetcode.com/problems/longest-valid-parentheses/
struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ml = 0;
        let mut stack: Vec<i32> = Vec::with_capacity(s.len());
        stack.push(-1);
        for (pos, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(pos as i32);
                continue;
            }
            if stack.pop().is_some() && !stack.is_empty() {
                ml = ml.max(pos as i32 - stack.last().unwrap());
                continue;
            } 
            stack.push(pos as i32);
        }
        ml
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, i32)> {
        vec![
            ("(()".to_string(), 2), (")()())".to_string(), 4), ("".to_string(), 0), ("()(()".to_string(),2), ("(((".to_string(),0), (")))".to_string(),0), ("()()()".to_string(),6), ("(()()())()()))()()(((()))))".to_string(),12), ("".to_string(),0), ("()(()()".to_string(),4), ("())()()((()()()))()()()()()()())))(()(())))()())()()()())(((()))))()()())()()()()())))(".to_string(),28)
        ]
    }
    #[test]
    fn test_longest_valid_parentheses() {
        for (s, want) in testcases() {
            assert_eq!(Solution::longest_valid_parentheses(s), want);
        }
    }
}

/* 
 * Alternate solutions
 *
impl Solution {
    /*
        replace valid parentheses with '+' is original sting
        example:
                "}}{}}([]){()" ==> will turn into ==> "}}++}++++{++"
    */
    fn reformat_string(s: &mut Vec<u8>) {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..s.len() {
            if let Some(&top_element) = stack.last() {
                if (s[i] == b'}' && s[top_element] == b'{')
                    || (s[i] == b']' && s[top_element] == b'[')
                    || (s[i] == b')' && s[top_element] == b'(')
                {
                    s[top_element] = b'+';
                    s[i] = b'+';
                    stack.pop();
                    continue;
                }
            }
            stack.push(i);
        }
    }

    fn max_continuous_substr_of_plus(s: Vec<u8>) -> i32 {
        let (mut max, mut curr) = (0, 0);
        for ch in s {
            if (ch == b'+') {
                curr += 1;
            } else {
                curr = 0;
            }
            if (curr > max) {
                max = curr;
            }
        }
        max
    }
    pub fn longest_valid_parentheses(mut s: String) -> i32 {
        let mut s = s.into_bytes();
        Self::reformat_string(&mut s);
        Self::max_continuous_substr_of_plus(s)
    }
}

//Alternate 2
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut output = 0;
        for (i, c) in s.chars().enumerate() {
            if c == ')' { continue }
            let mut p = 1;
            for (i, c) in s[i+1..].chars().enumerate() {
                p += if c == '(' { 1 } else { -1 };
                if p == 0 {
                    output = output.max(i+2);
                } else if p == -1 {
                    break;
                }
            }
        }
        output as i32
    }
}
*/
