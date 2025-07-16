/* Easy
 * https://leetcode.com/problems/merge-strings-alternately/
 */
#![feature(test)]
struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut p1 = word1.chars();
        let mut p2 = word2.chars();
        let mut result = String::with_capacity(word1.len() + word2.len());

        while let Some(c1) = p1.next() {
            result.push(c1);
            if let Some(c2) = p2.next() {
                result.push(c2);
            }
        }
        while let Some(c) = p1.next() {
            result.push(c);
        }
        while let Some(c) = p2.next() {
            result.push(c);
        }
        result
    }
}

struct SolutionAlt;
impl SolutionAlt {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut p1 = word1.chars();
        let mut p2 = word2.chars();
        let mut result = String::with_capacity(word1.len() + word2.len());

        while let Some(c1) = p1.next() {
            result.push(c1);
            if let Some(c2) = p2.next() {
                result.push(c2);
            }
        }
        while let Some(c) = p1.next() {
            result.push(c);
        }
        while let Some(c) = p2.next() {
            result.push(c);
        }
        result
    }
}

//Other Solutions
struct SolutionOther;
impl SolutionOther {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        //convert the strings to iterators
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        //create the result string the merged output
        let mut result = String::with_capacity(word1.len() + word2.len());

        //merge strings alternately
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => {
                    result.push(c1);
                }
                (None, Some(c2)) => {
                    result.push(c2);
                }
                (None, None) => break,
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}

fn test_cases() -> Vec<(String, String, String)> {
    vec![
        (
            String::from("abc"),
            String::from("pqr"),
            String::from("apbqcr"),
        ),
        (
            String::from("ab"),
            String::from("pqrs"),
            String::from("apbqrs"),
        ),
        (
            String::from("abcd"),
            String::from("pq"),
            String::from("apbqcd"),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{test_cases, Solution, SolutionAlt, SolutionOther};

    #[test]
    fn test_merge_alternately() {
        for (word1, word2, res) in test_cases() {
            assert_eq!(res, Solution::merge_alternately(word1, word2));
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_mysolution(b: &mut Bencher) {
        b.iter(|| {
            for (word1, word2, res) in test_cases() {
                assert_eq!(res, Solution::merge_alternately(word1, word2));
            }
        });
    }
    #[bench]
    fn bench_mysolution_alt(b: &mut Bencher) {
        b.iter(|| {
            for (word1, word2, res) in test_cases() {
                assert_eq!(res, SolutionAlt::merge_alternately(word1, word2));
            }
        });
    }

    #[bench]
    fn bench_othersolution(b: &mut Bencher) {
        b.iter(|| {
            for (word1, word2, res) in test_cases() {
                assert_eq!(res, SolutionOther::merge_alternately(word1, word2));
            }
        });
    }
}
