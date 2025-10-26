//https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter
struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let n_1 = s.len() - 1;
        let k = k as usize;
        let mut repetition = repetition;
        let mut stack = Vec::<char>::with_capacity(k);
        let mut letter_count = 0;
        s.chars().for_each(|c| {
            if c == letter {
                letter_count+=1;
            }
        });
        for (i, c) in s.chars().enumerate() {
            while !stack.is_empty()
                && stack[stack.len() - 1] > c
                && (stack.len() + n_1 - i) >= k
                && (stack[stack.len() - 1] != letter || repetition < letter_count)
            {
                if stack[stack.len() - 1] == letter {
                    //Dont count back to repetition until excess are spilled out
                    repetition += 1;
                }
                stack.pop();
            }
            if c == letter {
                if stack.len() < k {
                    //If done required repetition then count on the excess
                    repetition -= 1;
                    stack.push(c);
                }
                letter_count -= 1;
            } else if stack.len() as i32 + 0.max(repetition) < k as i32 {
                stack.push(c);
            }
        }
        String::from_iter(stack)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(String, i32, char, i32, String)> {
        vec![
            (
                "aeejfghhhhiilllstuvvwjwx".to_string(),
                3,
                'j',
                2,
                "ajj".to_string(),
            ),
            ("leet".to_string(), 3, 'e', 1, "eet".to_string()),
            ("leetcode".to_string(), 4, 'e', 2, "ecde".to_string()),
            ("bb".to_string(), 2, 'b', 2, "bb".to_string()),
            (
                "xxxxvvuuosqppppooommmlkjjoihgffddcccbbacdefijlommnnopprtvvwxxyyz".to_string(),
                8,
                'o',
                3,
                "jjoacdoo".to_string(),
            ),
            ("bba".to_string(), 1, 'b', 1, "b".to_string()),
            ("hjjhhhmhhwhz".to_string(), 6, 'h', 5, "hhhhhh".to_string()),
        ]
    }

    use super::Solution;
    #[test]
    fn test_smallest_subsequence() {
        for (s, k, letter, repetition, want) in testcases() {
            assert_eq!(
                Solution::smallest_subsequence(s, k, letter, repetition),
                want
            );
        }
    }
}
