#![feature(test)]
/* Hard
 * https://leetcode.com/problems/regular-expression-matching/
 */

struct Solution;

impl Solution {
    fn dfs(s: &Vec<u8>, p: &Vec<u8>, i: usize, j: usize, mem_table: &mut Vec<Vec<i32>>) -> bool {
        println!("{j} {i}");
        if i >= p.len() {
            if j >= s.len() {
                return true;
            }
            return false;
        }

        if mem_table[i][j] != -1 {
            println!(" Mem-> {j} {i}");
            return match mem_table[i][j] {
                0 => false,
                _ => true,
            };
        }

        let m = j < s.len() && (p[i] == s[j] || p[i] as char == '.');

        println!(" {j} {i} -> Match -> {m}");
        //Check if next is Kleene star
        if (i + 1) < p.len() && p[i + 1] as char == '*' {
            println!(" * -> ");
            //If yes then
                let r = Solution::dfs(s, p,i + 2, j, mem_table);
                println!("  {j} {i} ->  Right -> {r}");
                let l = m && Solution::dfs(s, p, i, j + 1, mem_table);
                println!("  {j} {i} ->  Left -> {l} Right -> {r}");
            return 
                //Either Skip it entirely (i.e, Zero Match and proceed to next character in Pattern)
                /*
                Solution::dfs(s, p,i + 2, j, mem_table);
                ||
                //Or Use it to make One Match and Proceed to see if next also Matches in Text
                (m && Solution::dfs(s, p, i, j + 1, mem_table)) 
                */
                match l || r {
                true => {
                    mem_table[i][j] = 1;
                    true
                },
                false =>  {
                    mem_table[i][j] = 0;
                    false
                }
            };
        }

        //If it wasn't Kleene star, let proceed with Character by Character Match instead
        println!(" {j} {i} -> Match -> {m} .|c -> ");
        match m {
            true => match Solution::dfs(s, p, i + 1, j + 1, mem_table) {
                true => {
                    mem_table[i][j] = 1;
                    true
                }
                false => {
                    mem_table[i][j] = 0;
                    false
                }
            },
            false => {
                mem_table[i][j] = 0;
                false
            }
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes().to_vec(), p.as_bytes().to_vec());
        let mut mem_table = vec![vec![-1; s.len() + 1]; p.len()];
        Solution::dfs(&s, &p, 0, 0, &mut mem_table)
    }
}

struct SolutionNoMemo {}

impl SolutionNoMemo {
    fn dfs(s: &Vec<u8>, p: &Vec<u8>, i: usize, j: usize, mem_table: &mut Vec<Vec<i32>>) -> bool {
        if j >= s.len() && i >= p.len() {
            return true;
        }

        if i >= p.len() {
            return false;
        }

        let m = j < s.len() && (p[i] == s[j] || p[i] as char == '.');

        if (i + 1) < p.len() && p[i + 1] as char == '*' {
            return SolutionNoMemo::dfs(s, p, i + 2, j, mem_table)
                || (m && SolutionNoMemo::dfs(s, p, i, j + 1, mem_table));
        }

        m && SolutionNoMemo::dfs(s, p, i + 1, j + 1, mem_table)
    }

    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes().to_vec(), p.as_bytes().to_vec());
        let mut mem_table = vec![vec![-1; s.len() + 1]; p.len()];
        SolutionNoMemo::dfs(&s, &p, 0, 0, &mut mem_table)
    }
}

//Other best Solution
/*

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut q = Vec::with_capacity(p.len());
        for c in p {
            if *c == b'*' && q.last() == Some(&b'*') {
                continue;
            } else {
                q.push(*c);
            }
        }

        let n = s.len();
        let mut m = p.len();

        let mut g = vec![vec![false; n + 1]; m + 1];
        g[m][n] = true;

        while m > 0 {
            m -= 1;

            let t = m > 0 && p[m] == b'*';
            m -= t as usize;

            for x in 0..=n {
                if !t {
                    g[m][x] = x < n && (p[m] == b'.' || s[x] == p[m]) && g[m + 1][x + 1];
                    continue;
                }

                let mut y = x;
                loop {
                    g[m][x] |= g[m + 2][y];
                    if g[m][x] {
                        break;
                    }

                    if y < n && (p[m] == b'.' || s[y] == p[m]) {
                        y += 1;
                        continue;
                    }

                    break;
                }
            }
        }

        g[0][0]
    }
}

*/
struct SolutionOldNotWorking {}

impl SolutionOldNotWorking {
    pub fn is_match(s: String, p: String) -> bool {
        let mut any = false;
        let mut first_char: u8 = ' ' as u8;
        let mut i = 0;
        let mut j = 0;
        while j < p.len() && i < s.len() {
            match p.as_bytes()[j] as char {
                '.' => {
                    any = true;
                    j += 1;
                    i += 1;
                }
                '*' => {
                    if !any && s.as_bytes()[i] != first_char {
                        j += 1;
                    }
                    if !any && s.as_bytes()[i] == first_char {
                        i += 1;
                        if i == s.len() {
                            j += 1;
                        }
                    }
                    if any {
                        i += 1;
                        j += 1;
                    }
                }
                'a'..='z' => {
                    if s.as_bytes()[i] == p.as_bytes()[j] {
                        first_char = s.as_bytes()[i];
                        i += 1;
                        j += 1;
                    } else {
                        j += 1;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        j == p.len() && i == s.len()
    }
}

fn main() {
    for (i, (s, p, r)) in test_cases().into_iter().enumerate() {
        if i == 9 {
            println!("TestCase #{i} {s} {p}");
            println!("Expected {} -> Got {}", r, Solution::is_match(s, p));
        }
    }
}

fn test_cases() -> Vec<(String, String, bool)> {
    vec![
        ("aa".to_string(), "a".to_string(), false),
        ("aa".to_string(), "a*".to_string(), true),
        ("ab".to_string(), ".*".to_string(), true),
        ("mississippi".to_string(), "mis*is*ip*.".to_string(), true),
        ("ab".to_string(), ".*c".to_string(), false),
        ("aa".to_string(), "aa".to_string(), true),
        ("aab".to_string(), "c*a*b".to_string(), true),
        ("aaa".to_string(), "a*a".to_string(), true),
        ("a".to_string(), "ab*".to_string(), true),
        ("abc".to_string(), "a*a*b*cd".to_string(), false),
        ("aaa".to_string(), "aaaa".to_string(), false),
        (
            "aaaaaaaaaaaaaaaaaaa".to_string(),
            "a*a*a*a*a*a*a*a*a*b".to_string(),
            false,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{test_cases, Solution, SolutionNoMemo};

    #[test]
    fn test_match() {
        for (i, (s, p, r)) in test_cases().into_iter().enumerate() {
            println!("TestCase #{i}");
            assert_eq!(r, Solution::is_match(s, p));
        }
    }
    #[test]
    fn test_match_nomemo() {
        for (i, (s, p, r)) in test_cases().into_iter().enumerate() {
            println!("TestCase #{i}");
            assert_eq!(r, SolutionNoMemo::is_match(s, p));
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_match(b: &mut Bencher) {
        b.iter(|| {
            for (i, (s, p, r)) in test_cases().into_iter().enumerate() {
                assert_eq!(r, Solution::is_match(s, p));
            }
        });
    }
    #[bench]
    fn bench_match_nomemo(b: &mut Bencher) {
        b.iter(|| {
            for (i, (s, p, r)) in test_cases().into_iter().enumerate() {
                assert_eq!(r, SolutionNoMemo::is_match(s, p));
            }
        });
    }
}
/*
 *
$ rustup run nightly cargo bench

running 4 tests
test tests::test_match ... ignored
test tests::test_match_nomemo ... ignored
test tests::bench_match        ... bench:      31,981.00 ns/iter (+/- 1,398.34)
test tests::bench_match_nomemo ... bench:  21,264,387.50 ns/iter (+/- 315,676.63)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out; finished in 12.46s

*/
