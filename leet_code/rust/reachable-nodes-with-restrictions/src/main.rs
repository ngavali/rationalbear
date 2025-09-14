//https://leetcode.com/problems/reachable-nodes-with-restrictions

struct Solution;

#[derive(Debug)]
struct DSU {
    parent: Vec<i32>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(s: i32) -> Self {
        DSU {
            parent: (0..s).collect(),
            rank: vec![1; s as usize],
        }
    }

    fn find_set(&mut self, v: i32) -> i32 {
        if v != self.parent[v as usize] {
            self.parent[v as usize] = self.find_set(self.parent[v as usize]);
        }
        self.parent[v as usize]
    }

    fn union_sets(&mut self, (mut a, mut b): (i32, i32)) {
        (a, b) = (self.find_set(a), self.find_set(b));
        if a != b {
            if self.rank[a as usize] > self.rank[b as usize] {
                (a, b) = (b, a);
            }
            self.parent[a as usize] = b;
            self.rank[b as usize] += self.rank[a as usize];
        }
    }

    fn get_size(&mut self, a: i32) -> i32 {
        let x = self.find_set(a);
        self.rank[x as usize]
    }
}

use std::collections::VecDeque;

impl Solution {
    fn dfs(
        node: usize,
        adj: &Vec<Vec<usize>>,
        is_visited: &mut Vec<bool>,
        visited: &mut i32,
    ) {
        is_visited[node] = true;
        *visited += 1;
        for &next in adj[node].iter() {
            if !is_visited[next] {
                Self::dfs(next, adj,  is_visited, visited);
            }
        }
    }

    fn bfs(
        node: usize,
        adj: &Vec<Vec<usize>>,
        is_visited: &mut Vec<bool>,
        visited: &mut i32,
    ) {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            let curr_node = queue.pop_front().unwrap();
            println!(" curr_node -> {curr_node}");
            *visited += 1;
            for &next in adj[curr_node].iter() {
                if !is_visited[next] {
                    is_visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut dsu = DSU::new(n);
        let mut restricted = restricted;
        restricted.sort();

        for edge in edges.iter() {
            let e = (edge[0], edge[1]);
            if !restricted.binary_search(&e.0).is_ok() && !restricted.binary_search(&e.1).is_ok() {
                dsu.union_sets(e);
            }
        }
        dsu.get_size(0)
    }
    pub fn reachable_nodes_other(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut restricted = restricted;
        restricted.sort();

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for e in edges.iter() {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }

        let mut is_visited = vec![false; n];
        is_visited[0] = true;
        for &node in restricted.iter() {
            is_visited[node as usize] = true;
        }
        let mut visited = 0;
        //Self::dfs(0, &adj,  &mut is_visited, &mut visited);
        Self::bfs(0, &adj,  &mut is_visited, &mut visited);
        println!( "{edges:?} \n ->restricted = {restricted:?}\n-> adj = {adj:?}\n-> is_visited = {is_visited:?}\n-> visited = {visited}");
        visited
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(i32, Vec<Vec<i32>>, Vec<i32>, i32)> {
        vec![
            (
                10,
                vec![
                    vec![4, 1],
                    vec![1, 3],
                    vec![1, 5],
                    vec![0, 5],
                    vec![3, 6],
                    vec![8, 4],
                    vec![5, 7],
                    vec![6, 9],
                    vec![3, 2],
                ],
                vec![2, 7],
                8,
            ),
            (
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6],
                ],
                vec![4, 5],
                4,
            ),
            (
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 5],
                    vec![0, 4],
                    vec![3, 2],
                    vec![6, 5],
                ],
                vec![4, 2, 1],
                3,
            ),
        ]
    }

    #[test]
    fn test_reachable_nodes() {
        for (n, edges, restricted, want) in testcases() {
            assert_eq!(Solution::reachable_nodes(n, edges, restricted), want);
        }
    }
}
