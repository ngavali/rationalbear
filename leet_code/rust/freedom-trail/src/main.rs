#![feature(test)]
/* Hard
 * https://leetcode.com/problems/freedom-trail/
 */

struct SolutionDp;
struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

impl SolutionDp {
    // Use HashMap for memo
    /*
    fn try_lock(
        key: &[u8],
        key_index: usize,
        ring_index: usize,
        ring_length: usize,
        ring_index_map: &HashMap<&u8, Vec<usize>>,
        memo_o: &mut Vec<Vec<usize>>,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if key_index == key.len() {
            return 0;
        }
        if let Some(&steps) = memo.get(&(key_index, ring_index)) {
            return steps;
        }
        /*
        if memo[key_index][ring_index] != usize::MAX {
            return memo[key_index][ring_index];
        }*/
        //Evaluate all possibilities when same letter at multiple positions
        for &next_index in ring_index_map.get(&key[key_index]).unwrap().iter() {
            let steps = ring_index
                .abs_diff(next_index)
                .min(ring_length - ring_index.abs_diff(next_index))
                + Solution::try_lock(
                    &key,
                    key_index + 1,
                    next_index,
                    ring_length,
                    ring_index_map,
                    memo_o,
                    memo,
                )
                + 1;

            memo.entry((key_index, ring_index))
                .and_modify(|step| *step = steps.min(*step))
                .or_insert(steps);

            /*
            memo[key_index][ring_index] = memo[key_index][ring_index].min(
                 // +1 for Button Press
            );
            */
        }
        *memo.get(&(key_index, ring_index)).unwrap()
        //memo[key_index][ring_index]
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
        let mut memo_o = vec![vec![usize::MAX; ring.len()]; key.len()];
        let mut memo = HashMap::<(usize, usize), usize>::with_capacity(key.len());
        //println!("{ring:?} {key:?} {ring_index_map:#?}");
        Solution::try_lock(
            key,
            0,
            0,
            ring.len(),
            &ring_index_map,
            &mut memo_o,
            &mut memo,
        ) as i32
    }
    */

    //Use 2D Array for memo
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
            memo[key_index][ring_index] = memo[key_index][ring_index].min(
                ring_index
                    .abs_diff(next_index)
                    .min(ring_length - ring_index.abs_diff(next_index))
                    + SolutionDp::try_lock(
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
        SolutionDp::try_lock(key, 0, 0, ring.len(), &ring_index_map, &mut memo) as i32
    }
}

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (ring, key) = (ring.as_bytes(), key.as_bytes());
        let ring_length = ring.len();
        let mut ring_index_map = HashMap::<&u8, Vec<usize>>::with_capacity(ring.len());
        for (i, key) in ring.iter().enumerate() {
            ring_index_map
                .entry(key)
                .and_modify(|index| index.push(i))
                .or_insert(vec![i]);
        }

        let mut visited = HashSet::<(usize, usize, usize)>::with_capacity(ring.len());
        let mut prio_queue =
            BinaryHeap::<Reverse<(usize, usize, usize)>>::with_capacity(ring.len());

        prio_queue.push(Reverse((0, 0, 0)));

        while let Some(Reverse(node)) = prio_queue.pop() {
            if node.2 == key.len() {
                return (node.0 + key.len()) as i32;
            }

            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);
            /*
                        ring_index
            .abs_diff(next_index)
            .min(ring_length - ring_index.abs_diff(next_index))
                        */

            for &next_index in ring_index_map.get(&key[node.2]).unwrap() {
                prio_queue.push(Reverse((
                    node.0
                        + node
                            .1
                            .abs_diff(next_index)
                            .min(ring_length - node.1.abs_diff(next_index)),
                    next_index,
                    node.2 + 1,
                )));
            }
        }
        1
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
        ), //Time limit exceeded case for no memo in DP
        (
            String::from("ymziohvpwhzlzjlyqfgfcgpuubfcpzqjxtrdmjqnxilejziydqdwakddcexybdaptrxgiwjjwpveeenvyeetknqncqyfjslncbwj"),
            String::from("vezjqxjzfpypmixnzelcpoqrhygyjeygyujztbzwbidimgcydxfcqiesqedjlewcjunnpwdvkbapwdfcxejrtlkndahfvqqjlntw"),
            986
         ), //Time limit exceeded case for Djikstras
                    (
            String::from("frltwceetklxkspcirtqpulqpenvqhqllaabxalwtjvrlatepddmlqavkdpcbhiaysghkkyrcmxipfspgylcivkovoxjhqcgozwg"),
            String::from("targmlsctdpclpoceiczeyavlhcpndqtljbhfqhewkklhkakxyxvxrsqoudbikeawiqvplpkltgqjwvalgocrasitxrygmlppqvf"),
            981
         ), //Time limit exceeded case for Djikstras
            (
                String::from("jmpbvwdnqlgtmcqfkfwpnegkuryrbluhxbtlvwpmunskcbjqnowhoekmutbhbmaynfwlaraxojyjsztfuszmwqnyuvaxquwguyav"),
                String::from("awanetffbptxmsulrqcymplgwtrksosvqhawujuxkwoudhmrwyqgualuabmmwvbfqnbpyjgonvyyncjtwubnvlkjnhexfkqubmzz"),
                807
            ), //Time limit exceeded case for Djikstras
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution, SolutionDp};

    #[test]
    fn test_find_rotate_steps() {
        for (i1, i2, o) in testcases() {
            assert_eq!(o, Solution::find_rotate_steps(i1, i2));
        }
    }
    #[test]
    fn test_find_rotate_steps_dp() {
        for (i1, i2, o) in testcases() {
            assert_eq!(o, SolutionDp::find_rotate_steps(i1, i2));
        }
    }
    /*
    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_find_rotate_steps(b: &mut Bencher) {
        b.iter(|| {
            for (i1, i2, o) in testcases() {
                assert_eq!(o, SolutionDp::find_rotate_steps(i1, i2));
            }
        });
    }
    #[bench]
    fn bench_find_rotate_steps_dp(b: &mut Bencher) {
        b.iter(|| {
            for (i1, i2, o) in testcases() {
                assert_eq!(o, Solution::find_rotate_steps(i1, i2));
            }
        });
    }
    */
}
