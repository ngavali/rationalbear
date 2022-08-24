struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut any = false;
        let mut first_char : u8 = ' ' as u8;
        let mut i = 0;
        let mut j = 0;
        while j < p.len() && i < s.len() {
            match p.as_bytes()[j] as char {
                '.' => { 
                        any = true;
                        j+=1;
                        i+=1;
                    },
                '*' => {
                        if !any && s.as_bytes()[i] != first_char {
                                j+=1;
                        }
                        if !any && s.as_bytes()[i] == first_char {
                            i+=1;
                            if i == s.len() {
                                j+=1;
                            }
                        }
                        if any {
                            i+=1;
                            j+=1;
                        }
                },
                'a'..='z' => {
                    if s.as_bytes()[i] == p.as_bytes()[j] {
                        first_char = s.as_bytes()[i];
                        i+=1;
                        j+=1;
                    } else {
                        j+=1;
                    }
                },
                _ => {
                    return false;
                }
            }
        }
        j == p.len() && i == s.len()
    }
}

fn main() {
    /**/
    assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
    assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
    assert_eq!(true, Solution::is_match("ab".to_string(), ".*".to_string()));
    assert_eq!(true, Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()));
    assert_eq!(false, Solution::is_match("ab".to_string(), ".*c".to_string()));
    assert_eq!(true, Solution::is_match("aa".to_string(), "aa".to_string()));
    assert_eq!(true, Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    assert_eq!(true, Solution::is_match("aaa".to_string(), "a*a".to_string()));
}
