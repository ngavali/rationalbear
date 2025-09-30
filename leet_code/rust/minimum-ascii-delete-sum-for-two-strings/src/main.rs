//https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/

struct Solution;

use std::collections::BinaryHeap;

impl Solution {

fn find_overlap_cost(
        s1: &Vec<char>,
        s1_idx: usize,
        s2: &Vec<char>,
        s2_idx: usize,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if s1_idx >= s1.len() || s2_idx >= s2.len() {
            return 0;
        }
        if memo[s1_idx][s2_idx] != -1 {
            return memo[s1_idx][s2_idx];
        }
        memo[s1_idx][s2_idx] = memo[s1_idx][s2_idx].max(
            match s1[s1_idx] == s2[s2_idx] {
                true => s1[s1_idx] as i32 
                        + Self::find_overlap_cost(
                            s1,
                            s1_idx + 1,
                            s2,
                            s2_idx + 1,
                            memo,
                        ),
                false => Self::find_overlap_cost(s1, s1_idx + 1, s2, s2_idx, memo).max(Self::find_overlap_cost(s1, s1_idx, s2, s2_idx + 1, memo)),
        });
        memo[s1_idx][s2_idx]
    }

    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        
        //We find overlapping characters and find cost of common characters
        //Remove that cost from total cost of all characters
        let mut s1_queue = BinaryHeap::new();
        let mut s2_queue = BinaryHeap::new();

        let mut s1_map = vec![Vec::with_capacity(s1.len()); 26];
        let mut s2_map = vec![Vec::with_capacity(s2.len()); 26];

        let s1: Vec<u8> = s1.as_bytes().to_vec();
        let s2: Vec<u8> = s2.as_bytes().to_vec();

        let mut total_cost = 0;

        for i in 0..s1.len() {
            s1_map[s1[i] as usize - 97].push(i);
            total_cost += s1[i] as i32;
        }
        for i in 0..s2.len() {
            s2_map[s2[i] as usize - 97].push(i);
            total_cost += s2[i] as i32;
        }
        for i in 0..s1_map.len() {
            if !s1_map[i].is_empty() && !s2_map[i].is_empty() {
                for &l in s1_map[i].iter() {
                    s1_queue.push((l, (i as u8 + 97) as char));
                }
                for &l in s2_map[i].iter() {
                    s2_queue.push((l, (i as u8 + 97) as char));
                }
            }
        }
        if s1_queue.is_empty() || s2_queue.is_empty() {
            return total_cost;
        }
        let s1q: Vec<char> = s1_queue
            .into_sorted_vec()
            .into_iter()
            .map(|(_, b)| b)
            .collect();
        let s2q: Vec<char> = s2_queue
            .into_sorted_vec()
            .into_iter()
            .map(|(_, b)| b)
            .collect();
        let mut memo = vec![vec![-1; s2q.len()]; s1q.len()];
        total_cost - 2 * Self::find_overlap_cost(&s1q, 0, &s2q, 0,  &mut memo)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(String, String, i32)> {
        vec![(
                "iwantthefox".to_string(),
                "idontwantthebee".to_string(),
                1070,
            ),
            ("nmnmajwmnmnmnmnmnmamsndmasnmdnamnwmndmawnmfwamgnwajkghwaijghanfnasmdnawhoirquoweruqworq".to_string(),"janskjdhajkdnasndnbjghjgabasdasdashdjaskdhasjdhasjkdhakshdssasdasdasdsa".to_string(), 11290),
            ("delete".to_string(), "leet".to_string(), 403),
            ("sea".to_string(), "eat".to_string(), 231),
            ("leetcode".to_string(), "codeforces".to_string(), 1068),
            ("ojweoiruw".to_string(), "mniahsvasf".to_string(), 1865),
            ("aaazzz".to_string(), "zzzaaa".to_string(), 582),
            (
                "xnbteodleejrzeo".to_string(),
                "gaouojqkkk".to_string(),
                2255,
            ),
            ("aaaaaaaaaaaa".to_string(), "bbbbb".to_string(), 1654),
        ]
    }

    use super::Solution;
    #[test]
    fn test_minimum_delete_sum() {
        for (s1, s2, want) in testcases() {
            /*
            println!(
                "--- {s1} - {s2}--- want -> {want}, got -> {}",
                Solution::minimum_delete_sum(s1.clone(), s2.clone())
            );
            */
            assert_eq!(Solution::minimum_delete_sum(s1, s2), want);
        }
    }
}
