struct Solution {}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        Vec::new()
    }
}


fn main() {

    let connections = vec![ vec![0,1], vec![1,2], vec![2,0], vec![1,3] ];
    Solution::critical_connections(4, connections);

    println!("Hello, world!");
}
