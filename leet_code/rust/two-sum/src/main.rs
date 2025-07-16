use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum( nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map : HashMap<&i32, usize> = HashMap::new();
        let mut i = 0;
        while i < nums.len() {
            match map.get(&(target-nums[i])) {
                Some(&v) => return vec![i as i32, v as i32],
                None => map.insert(&nums[i], i),
            };
            i+=1;
        }
        vec![]

        /*
        while i < N {
            match map.get(&(target-nums[i])) {
                None => {
                    map.insert(nums[i], i);
                },
            };
            i+=1;
        }
        res*/
    }
}

fn main() {
    /*
    println!("{:?}", Solution::fast_two_sum(vec![3,2,4], 6));
    println!("{:?}", Solution::fast_two_sum(vec![2,5,5,11], 10));
    println!("{:?}", Solution::fast_two_sum(vec![2,7,11], 9));
    */
    println!("{:?}", Solution::two_sum(vec![2,7,11,7,2], 9));
    println!("{:?}", Solution::two_sum(vec![3,2,4], 6));
    println!("{:?}", Solution::two_sum(vec![2,5,5,11], 10));
}
