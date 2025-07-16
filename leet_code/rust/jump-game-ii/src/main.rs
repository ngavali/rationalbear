struct Solution {
}

use std::collections::HashMap;
struct State {
    data: HashMap<usize, usize>
}

impl State {

    pub fn new() -> State {
        State{data: HashMap::new()}
    }

    pub fn get(&self, index: usize) -> Option<&usize> {
        self.data.get(&index)
    }

    pub fn insert(&mut self, index: usize, val: usize) {
        self.data.insert(index, val);
    }
}
    
pub fn Min(x: usize, y: usize) -> usize {
    if x < y {
        return x;
    }
    y
}


impl Solution {

    fn solve(state_data: &mut State, nums: &Vec<i32>, index: usize) -> usize {
        let (mut fj, mut i): (usize, usize)  = (0,0);

        if index < nums.len()-1 {
            fj = 10000;
            i = 0;
            while i < nums[index] as usize && index+1+i < nums.len() {
                fj = Min(
                    match state_data.get(index+i as usize +1) {
                        Some(&val) => val,
                        None => Solution::solve(state_data, &nums, 1+index+i),
                    }+1, fj );
                i+=1;
            }
        }
        state_data.insert(index, fj);
        fj
    }

    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut state_data = State::new();;
        Solution::solve(&mut state_data, &nums, 0) as i32
    }
}

fn main() {
    println!("{}", Solution::jump(vec![1,2,3]));
    /*
    println!("{}", Solution::jump(vec![3,2,1]));
    println!("{}", Solution::jump(vec![5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4, 1, 6, 8, 8, 4, 4, 2, 0, 3, 8, 5]));
    */
}
