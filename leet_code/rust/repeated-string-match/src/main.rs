#![feature(test)]

struct Solution;

impl Solution {
    fn compute_lps<'a>(s: &'a [u8], lps: &'a mut [usize]) {
        let mut l = 0;
        let mut i = 1;
        while i < s.len() {
            if s[i] == s[l] {
                lps[i] = lps[l] + 1;
                l = lps[i];
                i += 1;
                continue;
            }
            if l > 0 {
                l -= 1;
            } else {
                i += 1;
            }
        }
    }

    fn kmp(a: &[u8], b: &[u8], lps: &[usize]) -> usize {
        let (mut i, mut k) = (0, 0);
        while i < a.len() {
            if a[i] == b[k] {
                i += 1;
                k += 1;
                if k == b.len() {
                    return i;
                }
                continue;
            }
            if k > 0 {
                k = lps[k - 1];
            } else {
                i += 1;
            }
        }
        0
    }

    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let o_a = a.clone();
        let b = b.as_bytes();
        let mut a = a;
        let b_len = b.len();
        while a.len() < b_len {
            a.push_str(a.clone().as_str());
        }
        a.push_str(a.clone().as_str());
        let a = a.as_bytes();
        //Longest Proper Suffix
        let mut lps: Vec<usize> = vec![0; b_len];
        Solution::compute_lps(b, &mut lps);

        let p = Solution::kmp(&a, &b, &lps);
        match p {
            0 => -1,
            p => {
                (p / o_a.len()
                    + match p % o_a.len() {
                        0 => 0,
                        _ => 1,
                    }) as i32
            }
        }
    }

    //Brute force
    pub fn repeated_string_match_1(a: String, b: String) -> i32 {
        let (a_len, b_len) = (a.len(), b.len());
        let mut rounds = 0;
        let mut can_continue = true;
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let mut j = 0;

        for i in 0..a_len {
            can_continue = true;

            let mut k = i;
            while k < a_len {
                if b[j] != a[k] {
                    can_continue = false;
                    break;
                }
                j += 1;
                k += 1;
                if j == b_len {
                    return 1;
                }
            }

            while can_continue {
                rounds += 1;
                for k in 0..a_len {
                    if b[j] != a[k] {
                        can_continue = false;
                        break;
                    }
                    j += 1;
                    if j == b_len {
                        return rounds as i32 + 1;
                    }
                }
            }
            j = 0;
            rounds = 0;
        }
        -1
    }

    pub fn repeated_string_match_2(a: String, b: String) -> i32 {
        let a_len = a.len();
        let mut m_start;
        let mut rounds;
        let (a, b) = (a.as_bytes(), b.as_bytes());
        for i in 0..a_len {
            let ch_a = a[i];
            if b[0] == ch_a {
                rounds = 0;
                m_start = i;
                for (idx, &ch_b) in b.iter().enumerate() {
                    if ch_b != a[m_start] {
                        break;
                    }
                    m_start = (m_start + 1) % a_len;
                    if idx + 1 == b.len() {
                        return rounds as i32 + 1;
                    }
                    if m_start == 0 {
                        rounds += 1;
                    }
                }
            }
        }
        -1
    }

    //Understand this solution...
    pub fn repeated_string_match_learn_from_it(a: String, b: String) -> i32 {
        let ab = a.as_bytes();
        let bb = b.as_bytes();
        let mut ai = 0;
        let mut bi = 0;
        let mut lbi = 0;
        let mut llbi = 0;
        let mut ac = 1;
        loop {
            //println!("{} {} {}", ac, ai, bi);
            if ab[ai] == bb[bi] {
                ai += 1;
                bi += 1;
                if bi == bb.len() {
                    break;
                }
            } else {
                if ai + 1 >= bi {
                    ai = ai - bi + 1;
                } else {
                    if bi >= ab.len() {
                        return -1;
                    }
                    ac -= 1;
                    ai = ai + ab.len() - bi + 1;
                    lbi = llbi;
                }
                bi = 0;
            }
            if ai == ab.len() {
                if bi <= lbi {
                    return -1;
                }
                ai = 0;
                ac += 1;
                llbi = lbi;
                lbi = bi;
            }
        }
        ac
    }
}

fn main() {
    for (a, b, exp) in test_cases() {
        println!("E {exp} -> {}", Solution::repeated_string_match(a, b));
    }
}

pub fn test_cases() -> Vec<(String, String, i32)> {
    vec![
        (
            String::from("aaaaaaaaaaaaaaaaaaaaaab"),
            String::from("ba"),
            2,
        ),
        (String::from("a"), String::from("a"), 1),
        (String::from("aaacaaaa"), String::from("aaacaaaa"), 1),
        (String::from("abcd"), String::from("cabcdab"), -1),
        (String::from("abaa"), String::from("aabaa"), 2),
        (String::from("abc"), String::from("cabcabca"), 4),
        (String::from("abab"), String::from("aba"), 1),
        (String::from("a"), String::from("aa"), 2),
        (String::from("abcd"), String::from("cdabcdab"), 3),
        (String::from("abc"), String::from("wxyz"), -1),
        (String::from("xyz"), String::from("xyz"), 1),
        (String::from("xyz"), String::from("xy"), 1),
    ]
}

#[cfg(test)]

mod tests {
    use crate::Solution;

    fn test_cases() -> Vec<(String, String, i32)> {
        super::test_cases()
    }

    #[test]
    fn test_repeated_string_match() {
        for (i, (a, b, output)) in test_cases().into_iter().enumerate() {
            println!("Testcase #{}", i + 1);
            assert_eq!(output, Solution::repeated_string_match(a, b));
        }
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_solution_repeated_string_match(b: &mut Bencher) {
        b.iter(|| {
            for (i, (a, b, output)) in test_cases().into_iter().enumerate() {
                Solution::repeated_string_match(a, b);
            }
        });
    }
    #[bench]
    fn bench_solution_repeated_string_match_1(b: &mut Bencher) {
        b.iter(|| {
            for (i, (a, b, output)) in test_cases().into_iter().enumerate() {
                Solution::repeated_string_match_1(a, b);
            }
        });
    }
    #[bench]
    fn bench_solution_repeated_string_match_2(b: &mut Bencher) {
        b.iter(|| {
            for (i, (a, b, output)) in test_cases().into_iter().enumerate() {
                Solution::repeated_string_match_2(a, b);
            }
        });
    }
    #[bench]
    fn bench_solution_repeated_string_learn_from_it(b: &mut Bencher) {
        b.iter(|| {
            for (i, (a, b, output)) in test_cases().into_iter().enumerate() {
                Solution::repeated_string_match_learn_from_it(a, b);
            }
        });
    }
}
