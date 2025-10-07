//https://leetcode.com/problems/palindrome-partitioning/

struct Solution;

use std::collections::HashMap;
impl Solution {
    fn changes_to_make_palindrome(s: &[char]) -> i32 {
        if s.len() == 1 {
            return 0;
        }
        let mut changes = 0;
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                changes += 1;
            }
            i += 1;
            j -= 1;
        }
        changes
    }

    fn generate_semi_palindromes(s: &[char], l: usize, factor_map: &Vec<Vec<usize>>) -> i32 {
        let mut final_changes = s.len() as i32;
        for &factor in factor_map[l - 1].iter() {
            let mut groups = Vec::new();
            let mut c = 0;
            for d in 1..=factor {
                let mut group = Vec::new();
                let mut i = d;
                while i <= l {
                    group.push(s[i - 1]);
                    i += factor;
                }
                c += Self::changes_to_make_palindrome(&group);
                groups.push(group);
            }
            final_changes = final_changes.min(c);
        }
        final_changes
    }

    fn make_semi_palindrome(
        s: &[char],
        changes_map: &mut HashMap<String, i32>,
        factor_map: &Vec<Vec<usize>>,
    ) -> i32 {
        let ss = String::from_iter(s);
        if s.len() == 1 {
            return 0;
        }
        if s.len() == 2 {
            return Self::changes_to_make_palindrome(s);
        }
        let final_changes = match changes_map.get(&ss) {
            Some(&c) => c,
            None => {
                //Find factors
                let str_length = s.len();
                let c = Self::generate_semi_palindromes(&s, s.len(), factor_map);
                changes_map.insert(ss, c);
                c
            }
        };
        final_changes
    }

    fn generate_palindromic_partitions(
        start: usize,
        s: &[char],
        part: usize,
        changes_map: &mut HashMap<String, i32>,
        factor_map: &Vec<Vec<usize>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if memo[start][part] != 300 as i32 {
            return memo[start][part];
        }
        if part == 1 {
            let this_change = Self::make_semi_palindrome(&s[start..], changes_map, factor_map);
            return this_change;
        }
        let mut min_change = s.len() as i32;
        for k in start + 1..s.len() - 2 {
            let this_change = Self::make_semi_palindrome(&s[start..=k], changes_map, factor_map);
            min_change = min_change.min(
                this_change
                    + Self::generate_palindromic_partitions(
                        k + 1,
                        s,
                        part - 1,
                        changes_map,
                        factor_map,
                        memo,
                    ),
            );
        }
        memo[start][part] = min_change;
        min_change
    }

    pub fn minimum_changes(s: String, k: i32) -> i32 {
        //Generate palindromic_partitions
        let s = s.chars().collect::<Vec<char>>();
        let mut curr_list: Vec<String> = Vec::new();
        let mut changes_map = HashMap::new();
        let mut memo = vec![vec![300; k as usize + 1]; s.len() + 1];
        //let mut factor_map: HashMap<usize,Vec<usize>> = HashMap::new();

        let mut factor_map = vec![vec![1]; s.len()];
        for num in 2..=s.len() {
            let mut i = num;
            while i + num <= s.len() {
                i += num;
                factor_map[i - 1].push(num);
            }
        }

        let min_change = Self::generate_palindromic_partitions(
            0,
            &s,
            k as usize,
            &mut changes_map,
            &factor_map,
            &mut memo,
        );
        min_change
    }
}

/*
use std::collections::HashMap;
impl Solution {
    fn is_palindrome(s: &[char], mut i: usize, mut j: usize) -> bool {
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }

    fn find_factors(num: usize) -> Vec<usize> {
        let mut factors = vec![1];
        let mut factor = 2;
        while factor * factor <= num {
            if num % factor == 0 {
                factors.push(factor);
                let f = num / factor;
                if f != factor {
                    factors.push(f);
                }
            }
            factor += 1;
        }
        factors.dedup();
        factors
    }

    fn changes_to_make_palindrome(s: &[char]) -> i32 {
        if s.len() == 1 {
            return 0;
        }
        if s.len() == 2 {
            return (s[0] != s[1]) as i32;
        }
        if s.len() == 3 {
            return (s[0] != s[2]) as i32;
        }
        let mut changes = 0;
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                changes += 1;
            }
            i += 1;
            j -= 1;
        }
        println!(
            "    Changes to make Palindrome -> {} {}",
            String::from_iter(s),
            changes
        );
        changes
    }

    fn generate_semi_palindromes(s: &[char], l: usize, factors: Vec<usize>) -> i32 {
        let mut final_groups = Vec::new();
        let str_length = l;
        let mut changes = 0;
        let mut final_changes = Vec::<i32>::new();
        //println!( " >>>> SEMI PAL -- {} len = {str_length} Factors -> {factors:?}", String::from_iter(s));
        for factor in factors {
            let mut groups = Vec::new();
            let mut c = 0;
            for d in 1..=factor {
                let mut group = Vec::new();
                let mut i = d;
                while i <= str_length {
                    group.push(s[i - 1]);
                    i += factor;
                }
                c += Self::changes_to_make_palindrome(&group);
                println!("   >>> Group found factor -> {d} -> {group:?} total group change ={c}");
                groups.push(group);
            }
            changes += c;
            //println!("        {groups:?} {c}");
            final_changes.push(c);
            final_groups.push(groups);
            println!("FINAL --- {final_changes:2?} {final_groups:2?}");
        }
        if final_groups != vec![vec![vec![]]] {
            println!("                      >>>> Group -> {final_groups:?}\nChanges required -> {changes}");
        }
        final_changes.into_iter().min().unwrap()
    }

    fn make_semi_palindrome(s: &[char], changes_map: &mut HashMap<String, i32>) -> i32 {
        let ss = String::from_iter(s);
        println!("                  -- MAKE SEMI -- {ss}");
        if s.len() == 1 {
            return 0;
        }
        if s.len() == 2 {
            println!(" Called here!! {ss}");
            return Self::changes_to_make_palindrome(s);
        }
        let final_changes = match changes_map.get(&ss) {
            Some(&c) => {
                println!("Found in map {ss} {c}");
                //println!("{c}");
                c
            }
            None => {
                //Find factors
                let str_length = s.len();
                let factors = Self::find_factors(str_length);
                println!("Length -> {str_length} Factors -> {factors:?}");
                let c = Self::generate_semi_palindromes(&s, s.len(), factors);
                changes_map.insert(ss.clone(), c);
                //println!("{c}");
                c
            }
        };
        println!("                  -- MAKE SEMI -- {ss} --> {final_changes}");
        final_changes
    }

    fn generate_palindromic_partitions(
        start: usize,
        s: &[char],
        part: usize,
        curr_list: &mut Vec<String>,
        changes: &mut Vec<i32>,
        total_changes: &mut i32,
        changes_map: &mut HashMap<String, i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if memo[start][part] != 200000 {
            return memo[start][part];
        }
        println!("C -> {curr_list:?} Start-> {start} Part->{part}");
        if part == 1 {
            curr_list.push(String::from_iter(&s[start..]));
            let this_change = Self::make_semi_palindrome(&s[start..], changes_map);
            changes.push(this_change);
            println!(
                " === {:2} {curr_list:?} === \n ===      {changes:?}",
                part - 1
            );
            let mut c = 0;
            for change in changes.iter() {
                c += *change;
            }
            *total_changes = c.min(*total_changes);
            println!("{this_change:2} {c:2}");
            if c == 0 {
                println!("  >  {curr_list:?}");
                println!("   > {changes:?}");
            }
            changes.pop();
            curr_list.pop();

            //memo[start][part] = this_change;

            return this_change;
        }
        let mut min_change = s.len() as i32;
        for k in start + 1..s.len() - 2 {
            //if Self::is_palindrome(s, start, k) {
            curr_list.push(String::from_iter(&s[start..(k + 1)]));
            let mut c = 0;
            for change in changes.iter() {
                c += *change;
            }
            let this_change = Self::make_semi_palindrome(&s[start..=k], changes_map);
            changes.push(this_change);
            //changes.push(Self::changes_to_make_palindrome(&s[start..=k]));
            min_change = min_change.min(
                this_change
                    + Self::generate_palindromic_partitions(
                        k + 1,
                        s,
                        part - 1,
                        curr_list,
                        changes,
                        total_changes,
                        changes_map,
                        memo,
                    ),
            );
            changes.pop();
            curr_list.pop();
            //}
        }
        memo[start][part] = min_change;
        min_change
    }

    pub fn minimum_changes(s: String, k: i32) -> i32 {
        //Generate palindromic_partitions
        println!("\n--- {s} {k} ---");
        let s = s.chars().collect::<Vec<char>>();
        let mut curr_list: Vec<String> = Vec::new();
        let mut changes = Vec::new();
        let mut total_changes = s.len() as i32;
        let mut changes_map = HashMap::new();
        let mut memo = vec![vec![200000; k as usize + 1]; s.len() + 1];
        let min_change = Self::generate_palindromic_partitions(
            0,
            &s,
            k as usize,
            &mut curr_list,
            &mut changes,
            &mut total_changes,
            &mut changes_map,
            &mut memo,
        );
        println!("min change = {min_change}",);
        //println!("{palindromic_partitions:?}");
        min_change
    }
}
*/

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, i32, i32)> {
        vec![
            ("cbacccbabcaa".to_string(), 1,3),
            ("ababaa".to_string(), 2, 0),
            ("abcabc".to_string(), 3, 3),
            ("abcac".to_string(), 2, 1),
            ("abcac".to_string(), 2, 1),
            ("abcabc".to_string(), 1, 0),
            ("aabbaa".to_string(), 3, 0),
            ("abcdef".to_string(), 2, 2),
            ("cbbcaacbccbacaaccbcabcaacacbababaccbbabccbcccacbcacbccbcbccbcbbaaaccbbabbaaa".to_string(), 4, 5),
            ("cbbcaacbccbacaaccbcabcaacacbababaccbbabccbcccacbcacbccbcbccbcbbaaaccbbabbaaaaccccacabaabcbaabaabaaccacbbaacbacbbabaaaaacacaaacccacbaabbcbbacaacccbabbcbbaacaaacabbccbabaabbacbbcbcaaabbaabbcabccccccc".to_string(), 85,22),
            ("dcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcbadcba".to_string(),100,100),
            ("pafgscafzievqnnlubopjhuffvpcixrkrbcrecmbhmjyvcjhyhfxhpclbtzumzxkzlkprbcrdqsvkfxcuranriuhocilcthyunnfflttsbxgcamyzffyjpogtmlinlrmsihdhrxpspmrqdcdyhdlmwwcugewwhstkkrasuxzprancbtoepsbkebctsqnynxvoltwvdnj".to_string(),97, 87),
        ]
    }

    #[test]
    fn test_partition() {
        for (s, k, want) in testcases().into_iter() {
            assert_eq!(Solution::minimum_changes(s, k), want);
        }
    }
}
