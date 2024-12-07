struct Solution;

impl Solution {
    fn parent(vertex: &Vec<usize>, node: i32) -> usize {
        let mut parent = node as usize;
        while vertex[parent] != parent {
            parent = vertex[parent];
        }
        parent
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut redundant_edges = Vec::<Vec<i32>>::new();
        let mut vertex: Vec<usize> = { 0..edges.len() + 1 }.map(|i| i).collect();

        for e in edges {
            let (u, v) = (
                Solution::parent(&vertex, e[0]),
                Solution::parent(&vertex, e[1]),
            );
            if u != v {
                vertex[v] = u;
            } else {
                redundant_edges.push(e);
            }
        }

        redundant_edges.pop().unwrap_or(Vec::<i32>::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn test_cases() -> Vec<(Vec<Vec<i32>>, Vec<i32>)> {
        vec![
            (vec![vec![1, 2], vec![1, 3], vec![2, 3]], vec![2, 3]),
            (
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![1, 4],
                    vec![1, 5],
                    vec![4, 5],
                ],
                vec![4, 5],
            ),
        ]
    }

    #[test]
    fn test_find_redundant_connection() {
        for (i, (input, output)) in test_cases().into_iter().enumerate() {
            println!("Testcase #{}", i + 1);
            assert_eq!(output, Solution::find_redundant_connection(input));
        }
    }
}
