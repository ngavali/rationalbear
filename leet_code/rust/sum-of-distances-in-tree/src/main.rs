//https://leetcode.com/problems/sum-of-distances-in-tree/
struct Solution;

impl Solution {
    fn dfs_pass_1(
        node: usize,
        parent_node: usize,
        distance: i32,
        tree: &Vec<Vec<usize>>,
        dp: &mut Vec<i32>,
        child_node_count: &mut Vec<i32>,
    ) {
        let next_node_depth = distance + 1;
        tree[node].iter().for_each(|&next_node| {
            if next_node != parent_node {
                Self::dfs_pass_1(next_node, node, next_node_depth, tree, dp, child_node_count);
                child_node_count[node] += child_node_count[next_node];
                dp[0] += next_node_depth;
            }
        });
    }

    fn dfs_pass_2(
        node: usize,
        parent_node: usize,
        nodes: i32,
        tree: &Vec<Vec<usize>>,
        dp: &mut Vec<i32>,
        child_node_count: &mut Vec<i32>,
        sum_at_prev_node: i32,
    ) {
        dp[node] = sum_at_prev_node;
        tree[node].iter().for_each(|&next_node| {
            if next_node != parent_node {
                Self::dfs_pass_2(
                    next_node,
                    node,
                    nodes,
                    tree,
                    dp,
                    child_node_count,
                    sum_at_prev_node + nodes - 2 * child_node_count[next_node]
                );
            }
        })
    }

    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut tree = vec![Vec::<usize>::with_capacity(n); n];
        let mut dp = vec![0; n];
        let mut child_node_count = vec![1; n as usize];
        edges.iter().for_each(|edge| {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        });
        Self::dfs_pass_1(0, n, 0, &tree, &mut dp, &mut child_node_count);
        let sum_at_zero = dp[0];
        Self::dfs_pass_2(0, n, n as i32, &tree, &mut dp, &mut child_node_count, sum_at_zero);
        dp
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, Vec<Vec<i32>>, Vec<i32>)> {
        vec![
            (
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
                vec![8, 12, 6, 10, 10, 10],
            ),
            (1, vec![], vec![0]),
            (2, vec![vec![1, 0]], vec![1, 1]),
            (3, vec![vec![2, 1], vec![0, 2]], vec![3, 3, 2]),
            (11, vec![vec![0,1],vec![0,2],vec![1,3],vec![3,4],vec![5,1],vec![6,2],vec![7,6],vec![7,8],vec![8,9],vec![8,10]], vec![28,31,27,38,47,40,28,31,36,45,45]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_sum_of_distances_in_tree() {
        for (n, edges, want) in testcases() {
            assert_eq!(Solution::sum_of_distances_in_tree(n, edges), want);
        }
    }
}
