struct Solution;

impl Solution {
    fn dfs(text_index: usize, pattern_index: usize, text: &[u8], pattern: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if pattern_index == pattern.len()
            || (pattern[pattern_index] == b'*' && pattern_index == pattern.len() - 1)
        {
            if text_index == text.len() {
                return true;
            }
        }
        if (text_index == text.len() && pattern_index < pattern.len()) || (pattern_index == pattern.len() && text_index < text.len()) {
            return false;
        }
        if let Some(res) = memo[pattern_index][text_index] {
            return res;
        }
        memo[pattern_index][text_index] = Some(match pattern[pattern_index] == text[text_index] || pattern[pattern_index] == b'?' {
            true => Solution::dfs(text_index + 1, pattern_index + 1, text, pattern, memo),
            false if pattern[pattern_index] == b'*' =>
                        Solution::dfs(text_index, pattern_index + 1, text, pattern, memo) ||
                        Solution::dfs(text_index + 1, pattern_index, text, pattern, memo),
            _ => false
        });
        return memo[pattern_index][text_index].unwrap();
    }
    pub fn is_match(s: String, p: String) -> bool {
        let mut ct = false;
        let p = p
            .as_bytes()
            .into_iter().copied()
            .filter(|&c| {
                    if c != b'*' || !ct  {
                    ct = c == b'*' ;
                    return true;
                }
                false
            })
            .collect::<Vec<u8>>();
        let mut memo = vec![vec![None; s.len()]; p.len()];
        Solution::dfs(0, 0, s.as_bytes(), &p, &mut memo)
    }
}

fn main() {
    println!("Hello, world!");
}

fn testcases() -> Vec<(String, String, bool)> {
    vec![
        (String::from("abbaabbbbababaababababbabbbaaaabbbbaaabbbabaabbbbbabbbbabbabbaaabaaaabbbbbbaaabbabbbbababbbaaabbabbabb"), String::from("***b**a*a*b***b*a*b*bbb**baa*bba**b**bb***b*a*aab*a**"), true),
        (String::from("aa"), String::from("a*"), true),
        (String::from("aa"), String::from("aa"), true),
        (
            String::from("mississippi"),
            String::from("m??*ss*?i*pi"),
            false,
        ),
        (String::from("aa"), String::from("a"), false),
        (String::from("aa"), String::from("*"), true),
        (String::from("cb"), String::from("?a"), false),
        (String::from("adceb"), String::from("*a*b"), true),
        (String::from("acdcb"), String::from("a*c?b"), false),
        (String::from(""), String::from("******"), true),
        
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_is_match() {
        for (i, (s, p, exp_o)) in testcases().into_iter().enumerate() {
            println!("Testcase#{i}");
            assert_eq!(exp_o, Solution::is_match(s, p));
        }
    }
}
