//https://leetcode.com/problems/assign-elements-to-groups-with-constraints/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_divisor(group: i32, elements_map: &HashMap<i32, i32>) -> i32 {
        let mut res = 9999999;
        let mut found = false;
        let sqrt = group.isqrt() + 1;
        for num in 1..sqrt {
            println!("{group} {num} {sqrt}");
            if group % num == 0 {
                let mul = group / num;
                println!(" {group} = {num} x {mul}");
                if let Some(&idx) = elements_map.get(&num) {
                    println!(" Found -> {num} -> {idx} {res}");
                    res = res.min(idx);
                    found = true;
                }
                if let Some(&idx) = elements_map.get(&mul) {
                    println!(" Found -> {mul} -> {idx} {res}");
                    res = res.min(idx);
                    found = true;
                }
            }
        }
        match found {
            true => res,
            false => -1,
        }
    }
    
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let largest = *groups.iter().max().unwrap_or(& 0);
        let mut assignments = vec![-1; largest as usize +1];
        elements.iter().enumerate().for_each(|(idx, &num)| {
            if num > largest || assignments[num as usize] != -1 {
                return;
            }
            (num..=largest).step_by(num as usize).for_each(|m|
                if assignments[m as usize] == -1 {
                assignments[m as usize] = idx as i32;
            });
        });
        groups
            .iter()
            .map(|&group| assignments[group as usize])
            .collect()
    }
    pub fn assign_elements_alt1(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let largest = *groups.iter().max().unwrap_or(& 0) ;
        let mut assignment: HashMap<i32, i32> = HashMap::new(); // groups.iter().map(|&group| (group, -1)).collect();
        let mut seen = HashMap::new();
        elements.iter().enumerate().for_each(|(idx, num)| {
            if *num < largest && seen.get(num) == None {
            for &group in groups.iter() {
                if assignment.get(&group) == None && group % num == 0 {
                    assignment.insert(group, idx as i32);
                }
            }
            seen.insert(num, true);
            }
        });

        groups
            .into_iter()
            .map(|num| *assignment.get(&num).unwrap_or(& -1))
            .collect()
    }
    pub fn assign_elements_alt2(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut assignment = HashMap::<i32, i32>::new();
        let mut elements_map: HashMap<i32, i32> = HashMap::new();
        elements.iter().enumerate().for_each(|(idx, num)| {
            if let Some(idxs) = elements_map.get_mut(num) {
                *idxs = (idx as i32).min(*idxs);
            } else {
                elements_map.insert(*num, idx as i32);
            }
        });
        println!("Map -> {elements_map:?} ");
        for group in groups {
            if let Some(&idx) = assignment.get(&group) {
                println!("Cached: group {group} -> {idx}");
                result.push(idx);
            } else {
                let res = Solution::find_divisor(group, &elements_map);
                println!("  group {group} -> {res}");
                assignment.insert(group, res);
                result.push(res);
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    fn testcases() -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> {
        vec![
            (vec![8, 4, 3, 2, 4], vec![4, 2], vec![0, 0, -1, 1, 0]),
            (vec![2, 3, 5, 7], vec![5, 3, 3], vec![-1, 1, 0, -1]),
            (vec![10, 21, 30, 41], vec![2, 1], vec![0, 1, 0, 1]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_assign_elements() {
        for (groups, elements, want) in testcases() {
            assert_eq!(Solution::assign_elements(groups, elements), want);
        }
    }
}
