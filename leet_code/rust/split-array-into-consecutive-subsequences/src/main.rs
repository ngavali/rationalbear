use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut numsFreqMap: HashMap<i32, i32> = HashMap::new();
        let mut nextInSQMap: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            match numsFreqMap.get_mut(&num) {
                Some(&mut (mut freq)) => freq += 1,
                None => {
                    numsFreqMap.insert(num, 1);
                }
            };
        }

        for &curr_num in nums.iter() {
            match numsFreqMap.get_mut(&curr_num) {
                Some(&mut (mut curr_freq)) => {
                    if curr_freq == 0 {
                        continue;
                    } else {
                        curr_freq -= 1;

                        let prev_num = curr_num - 1;

                        if let Some(&mut (mut prev_freq)) = nextInSQMap.get_mut(&prev_num) {
                            if prev_freq > 0 {
                                prev_freq -= 1;
                                match nextInSQMap.get_mut(&curr_num) {
                                    Some(&mut (mut seq_freq)) => seq_freq += 1,
                                    None => {
                                        nextInSQMap.insert(curr_num, 1);
                                    }
                                }
                            }
                        } else {
                            let next_num = curr_num + 1;
                            let next_to_next_num = next_num + 1;
                            
                            if let Some(&mut mut next_num_freq) = numsFreqMap.get_mut(&next_num) {
                                if next_num_freq > 0 {
                                    if let Some(&mut mut next_to_next_num_freq) = numsFreqMap.get_mut(&next_to_next_num) {
                                        next_num_freq -= 1;
                                        next_to_next_num_freq -= 1;
                                        match nextInSQMap.get_mut(&next_to_next_num) {
                                            Some(&mut (mut seq_freq)) => seq_freq += 1,
                                            None => {
                                                nextInSQMap.insert(next_to_next_num, 1);
                                            }
                                        };
                                    } else {
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }

                        }
                    }
                }
                None => {}
            }
        }

        println!("{:?} {:?}", numsFreqMap, nextInSQMap);
        true
    }
}

fn main() {
    println!("{}", Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    println!("{}", Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    println!("{}", Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
}
