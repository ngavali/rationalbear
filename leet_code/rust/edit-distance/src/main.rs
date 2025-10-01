//https://leetcode.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<u8> = word1.into_bytes();
        let word2: Vec<u8> = word2.into_bytes();
        let (m, n) = (word1.len(), word2.len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..=word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 0..=word2.len() {
            dp[0][j] = j as i32;
        }
        for i in 0..word1.len() {
            for j in 0..word2.len() {
                if word1[i] == word2[j] {
                    //If both characters are same its a NoOP
                    //We move forward
                    dp[i + 1][j + 1] = dp[i][j];
                } else {
                    //If they don't match then
                    //We either
                    //  Insert word2 character, makes j move forward
                    //  Replace word1 character with word2 charactor,
                    //      makes them i & j both move forward
                    //  Delete character from word1,
                    //      makes i move forward
                    dp[i + 1][j + 1] = 1 + dp[i + 1][j].min((dp[i][j]).min(dp[i][j + 1]));
                }
            }
        }
        dp[m][n]
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
