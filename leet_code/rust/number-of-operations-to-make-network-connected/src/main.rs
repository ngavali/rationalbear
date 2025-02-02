struct Solution;

use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Dsu {
            parent: { 0..size }.collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let (xset, yset) = (self.find(x), self.find(y));
        match self.rank[xset].cmp(&self.rank[yset]) {
            Ordering::Less => self.parent[xset] = yset,
            Ordering::Greater => self.parent[yset] = xset,
            Ordering::Equal => {
                self.parent[yset] = xset;
                self.rank[xset] += 1;
            }
        }
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut connected_components = n;
        let n = n as usize;
        if connections.len() < n - 1 {
            return -1;
        }

        let mut dsu = Dsu::new(n);

        for connection in connections {
            if dsu.find(connection[0] as usize) != dsu.find(connection[1] as usize) {
                connected_components -= 1;
                dsu.union(connection[0] as usize, connection[1] as usize);
            }
        }

        connected_components - 1
    }

    fn dfs(node: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
        if visited[node] {
            return;
        }

        visited[node] = true;

        for &neighbour in adj[node].iter() {
            Solution::dfs(neighbour, adj, visited);
        }
    }

    fn bfs(node: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
        let mut queue = VecDeque::<usize>::new();

        queue.push_back(node);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            visited[node] = true;
            for &neighbour in adj[node].iter() {
                if !visited[neighbour] {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    pub fn make_connected_graph(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() < n - 1 {
            return -1;
        }

        println!("{:?}", Dsu::new(n));
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut visited: Vec<bool> = vec![false; n];

        for connection in connections.iter() {
            adj[connection[0] as usize].push(connection[1] as usize);
            adj[connection[1] as usize].push(connection[0] as usize);
        }

        let mut connected_components = 0;
        for node in 0..n {
            if !visited[node] {
                connected_components += 1;
                Solution::bfs(node, &adj, &mut visited);
            }
        }
        println!("{connections:?}\n{adj:?}\n{visited:?}\n{connected_components}");
        connected_components - 1
    }
}
fn main() {
    for (i, (n, connections, exp_op)) in testcases().into_iter().enumerate() {
        println!("Testcase #{i}");
        assert_eq!(exp_op, Solution::make_connected(n, connections));
    }
}

fn testcases() -> Vec<(i32, Vec<Vec<i32>>, i32)> {
    vec![
        (6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]], -1),
        (4, vec![vec![0, 1], vec![0, 2], vec![1, 2]], 1),
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]],
            2,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_make_connected() {
        for (i, (n, connections, exp_op)) in testcases().into_iter().enumerate() {
            println!("Testcase #{i}");
            assert_eq!(exp_op, Solution::make_connected(n, connections));
        }
    }
}
