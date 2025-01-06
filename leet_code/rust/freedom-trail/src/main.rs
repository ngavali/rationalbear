/* Hard
 * https://leetcode.com/problems/freedom-trail/
 * TODO: Implement using Djikstras Algo
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn try_lock(
        key: &[u8],
        key_index: usize,
        ring_index: usize,
        ring_length: usize,
        ring_index_map: &HashMap<&u8, Vec<usize>>,
        memo: &mut Vec<Vec<usize>>,
    ) -> usize {
        if key_index == key.len() {
            return 0;
        }
        if memo[key_index][ring_index] != usize::MAX {
            return memo[key_index][ring_index];
        }
        for &next_index in ring_index_map.get(&key[key_index]).unwrap().iter() {
            //println!( " At -> keyIndex:{key_index} ringIndex:{ring_index} keyCharacter:{:?} => Ring character at keyIndex:{key_index} ringIndex:{ring_index} ringNextIndex:{next_index}", String::from_utf8(vec![key[key_index]]));
            memo[key_index][ring_index] = memo[key_index][ring_index].min(
                ring_index
                    .abs_diff(next_index)
                    .min(ring_length - ring_index.abs_diff(next_index))
                    + Solution::try_lock(
                        &key,
                        key_index + 1,
                        next_index,
                        ring_length,
                        ring_index_map,
                        memo,
                    )
                    + 1,
            );
        }

        memo[key_index][ring_index]
    }

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (ring, key) = (ring.as_bytes(), key.as_bytes());

        let mut ring_index_map = HashMap::<&u8, Vec<usize>>::with_capacity(ring.len());

        for (i, key) in ring.iter().enumerate() {
            ring_index_map
                .entry(key)
                .and_modify(|index| index.push(i))
                .or_insert(vec![i]);
        }

        let mut memo = vec![vec![usize::MAX; ring.len()]; key.len()];
        let min_steps = Solution::try_lock(key, 0, 0, ring.len(), &ring_index_map, &mut memo);

        //println!("{ring:?} {key:?} {ring_index_map:#?} {min_steps}");
        min_steps as i32
    }
}

fn main() {
    println!("Hello, world!");
}

fn testcases() -> Vec<(String, String, i32)> {
    vec![
        (String::from("godding"), String::from("gd"), 4),
        (String::from("godding"), String::from("godding"), 13),
        (
            String::from("caotmcaataijjxi"),
            String::from("oatjiioicitatajtijciocjcaaxaaatmctxamacaamjjx"),
            137,
        ), //Time limit exceeded case
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_find_rotate_steps() {
        for (i1, i2, o) in testcases() {
            assert_eq!(o, Solution::find_rotate_steps(i1, i2));
        }
    }
}
