struct Solution {}

impl Solution {
    pub fn dfs(adj: &Vec<Vec<i32>>, visited: &mut Vec<bool>, src: usize) {
        if !visited[src] {
            visited[src] = true;
            for nxt in &adj[src] {
                Solution::dfs(adj, visited, *nxt as usize);
            }
        }
    }
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adj: Vec<Vec<i32>> = vec![Vec::new();n as usize];
        let mut visited = vec![false; n as usize];
        let mut num_components = 0;
        for edge in edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }
    
        for src in 0..n {
            if !visited[src as usize] {
                Solution::dfs(&adj, &mut visited, src as usize);
                num_components += 1;
            }
        }
        num_components
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn test_count_components() {
        let testcases = vec![
            (5, vec![vec![0, 1], vec![1, 2], vec![3, 4]], 2),
            (5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], 1),
            (5, vec![vec![0,1],vec![1,2],vec![0,2],vec![3,4]], 2)
        ];

        for (n, edges, want) in testcases {
            assert_eq!(want, Solution::count_components(n, edges));
        }
    }
}
