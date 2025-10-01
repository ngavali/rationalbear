//https://leetcode.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<u8> = word1.into_bytes();
        let word2: Vec<u8> = word2.into_bytes();
        let (m, n) = (word1.len(), word2.len());
        let mut prev = vec![0; n + 1];
        let mut next = vec![0; n + 1];
        for j in 0..=word2.len() {
            prev[j] = j;
        }
        for i in 1..=m {
            next[0] = i;
            for j in 1..=n {
                //If both characters are same its a NoOP
                //We move forward
                next[j] = prev[j - 1];
                if word1[i - 1] != word2[j - 1] {
                    //If they don't match then
                    //We either
                    //  Insert word2 character, makes j move forward
                    //  Replace word1 character with word2 charactor,
                    //      makes them i & j both move forward
                    //  Delete character from word1,
                    //      makes i move forward
                    next[j] = 1 + next[j - 1].min(next[j].min(prev[j]));
                }
            }
            std::mem::swap(&mut next, &mut prev);
        }
        prev[n] as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(String, String, i32)> {
        vec![
            ("horse".to_string(), "ros".to_string(), 3),
            ("intention".to_string(), "execution".to_string(), 5),
            ("".to_string(), "".to_string(), 0),
            ("abc".to_string(), "defghi".to_string(), 6),
            ("leet".to_string(), "code".to_string(), 4),
            (
                "azertyqsdfghqwerty".to_string(),
                "azertyjklmqwerty".to_string(),
                6,
            ),
            (
                "aaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "aaaaaaaaaaaaaav".to_string(),
                8,
            ),
            ("aaayuio".to_string(), "hjklaaa".to_string(), 7),
        ]
    }

    #[test]
    fn test_min_distance() {
        for (word1, word2, want) in testcases() {
            assert_eq!(Solution::min_distance(word1, word2), want);
        }
    }
}
