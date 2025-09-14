//https://leetcode.com/problems/reachable-nodes-with-restrictions

struct Solution;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(s: usize) -> Self {
        Dsu {
            parent: (0..s).collect(),
            rank: vec![1;s],
        }
    }

    fn find_set_mut(&mut self, v: usize) -> usize {
        if v != self.parent[v] {
            self.parent[v] = self.find_set_mut(self.parent[v]);
        }
        self.parent[v]
    }

    fn find_set(&self, v: usize) -> usize {
        self.parent[v]
    }

    fn union_sets(&mut self, (mut a, mut b): (usize, usize)) {
        (a, b) = (self.find_set_mut(a), self.find_set_mut(b));
        if a != b {
            if self.rank[a] > self.rank[b] {
                self.parent[b] = a;
                self.rank[a] += self.rank[b];
            } else {
                self.parent[a] = b;
                self.rank[b] += self.rank[a];
            }
        }
    }

    fn get_size(&self, a: usize) -> usize {
        self.rank[self.find_set(a)]
    }
}

impl Solution {
    fn dfs(
        node: usize,
        adj: &[Vec<usize>],
        is_visited: &mut [bool],
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
        adj: &[Vec<usize>],
        is_visited: &mut [bool],
        visited: &mut i32,
    ) {
        let mut queue = Vec::new();
        queue.push(node);
        is_visited[node] = true;
        while let Some(curr_node) = queue.pop() {
            *visited += 1;
            for &next in adj[curr_node].iter() {
                if !is_visited[next] {
                    is_visited[next] = true;
                    queue.push(next);
                }
            }
        }
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
        for &node in restricted.iter() {
            is_visited[node as usize] = true;
        }
        let mut visited = 0;
        //Self::dfs(0, &adj,  &mut is_visited, &mut visited);
        Self::bfs(0, &adj,  &mut is_visited, &mut visited);
        visited
    }

    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut dsu = Dsu::new(n as usize);
        let mut restricted = restricted;
        restricted.sort();
        for edge in edges.iter() {
            if restricted.binary_search(&edge[0]).is_err() && restricted.binary_search(&edge[1]).is_err() {
                dsu.union_sets((edge[0] as usize, edge[1] as usize));
            }
        }
        dsu.get_size(0) as i32
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
