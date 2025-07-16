#![feature(test)]

/* Medium
 * https://leetcode.com/problems/longest-palindromic-substring/
 *
 * TODO: Mancher's Algorithm
 */

struct Solution3;

impl Solution3 {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut ans = (0, 0, 0);

        fn expand(mut i: i32, mut j: i32, s: &[u8]) -> (i32, i32, i32) {
            while i >= 0 && j < s.len() as i32 && s[i as usize] == s[j as usize] {
                i -= 1;
                j += 1;
            }
            (i + 1, j, j - i)
        }

        for i in 0..s.len() {
            let odd_palindrome = expand(i as i32, i as i32, &s);
            if odd_palindrome.2 > ans.2 {
                ans = odd_palindrome;
            }

            let even_palindrome = expand(i as i32, i as i32 + 1, &s);
            if even_palindrome.2 > ans.2 {
                ans = even_palindrome;
            }
        }

        String::from_utf8(s[ans.0 as usize..ans.1 as usize].to_vec()).unwrap()
    }
}

struct Solution2;

impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut ans = (0, 0);

        for i in 0..s.len() {
            dp[i][i] = true;
        }

        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                ans = (i, i + 1);
            }
        }

        for diff in 2..s.len() {
            for i in 0..s.len() - diff {
                let j = i + diff;
                if s[i] == s[j] && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    ans = (i, j);
                }
            }
        }

        String::from_utf8(s[ans.0..1 + ans.1].to_vec()).unwrap()
    }
}

struct Solution1 {}

impl Solution1 {
    fn is_palindrome(s: &[u8], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        let s = s.as_bytes();
        let s_len = s.len();
        let mut length = s_len;
        while length > 0 {
            for i in 0..(s_len - length + 1) {
                if Solution1::is_palindrome(s, i, i + length - 1) {
                    return String::from_utf8(s[i..i + length].to_vec()).unwrap();
                }
            }
            length -= 1;
        }
        String::from_utf8(vec![s[0]]).unwrap()
    }
}

fn main() {
    for (n, (i, o)) in testcases().into_iter().enumerate() {
        println!("Testcase#{n}");
        println!("\n {:?} {:?}", o, Solution1::longest_palindrome(i));
    }
}

fn testcases() -> Vec<(String, Vec<String>)> {
    vec![
        (
            String::from("babad"),
            vec![String::from("bab"), String::from("aba")],
        ),
        (String::from("cbbd"), vec![String::from("bb")]),
        (String::from("a"), vec![String::from("a")]),
        (String::from("ac"), vec![String::from("a")]),
        (String::from("aacabdkacaa"), vec![String::from("aca")]),
        (String::from("bb"), vec![String::from("bb")]),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution1, Solution2, Solution3};

    #[test]
    fn test_solution1_is_palindrome() {
        for (i, o) in testcases() {
            let mut ans = false;
            for out in o {
                ans = (out == Solution1::longest_palindrome(i.clone())) || ans;
            }
            assert_eq!(ans, true);
        }
    }
    fn test_solution2_is_palindrome() {
        for (i, o) in testcases() {
            let mut ans = false;
            for out in o {
                ans = (out == Solution2::longest_palindrome(i.clone())) || ans;
            }
            assert_eq!(ans, true);
        }
    }
    fn test_solution3_is_palindrome() {
        for (i, o) in testcases() {
            let mut ans = false;
            for out in o {
                ans = (out == Solution3::longest_palindrome(i.clone())) || ans;
            }
            assert_eq!(ans, true);
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solution1_is_palindrome(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                Solution1::longest_palindrome(i);
            }
        });
    }
    #[bench]
    fn bench_solution2_is_palindrome(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                Solution2::longest_palindrome(i);
            }
        });
    }
    #[bench]
    fn bench_solution3_is_palindrome(b: &mut Bencher) {
        b.iter(|| {
            for (i, o) in testcases() {
                Solution3::longest_palindrome(i);
            }
        });
    }
}
