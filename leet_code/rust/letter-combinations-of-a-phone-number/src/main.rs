//https://leetcode.com/problems/letter-combinations-of-a-phone-number/
struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letter_map = vec![
            vec![],
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut combinations = vec!["".to_string()];
        for &digit in digits.as_bytes() {
            let digit = digit as i32 - 48;
            let mut new_combination: Vec<String> = Vec::new();
            for &letter in letter_map[digit as usize].iter() {
                for com_idx in 0..combinations.len() {
                    new_combination.push(combinations[com_idx].clone() + &letter.to_string());
                    //new_combination.push(format!("{}{}", combinations[com_idx], letter.to_string()));
                }
            }
            combinations = new_combination;
        }
        match combinations.len() {
            1 => vec![],
            _ => combinations,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    fn testcases() -> Vec<(String, Vec<String>)> {
        vec![
            (
                "23".to_string(),
                vec![
                    "ad".to_string(),
                    "ae".to_string(),
                    "af".to_string(),
                    "bd".to_string(),
                    "be".to_string(),
                    "bf".to_string(),
                    "cd".to_string(),
                    "ce".to_string(),
                    "cf".to_string(),
                ],
            ),
            ("".to_string(), vec![]),
            (
                "2".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_letter_combinations() {
        for (digits, mut want) in testcases() {
            want.sort();
            let mut got = Solution::letter_combinations(digits);
            got.sort();
            assert_eq!(got, want);
        }
    }
}
