#![feature(test)]
struct Solution;

//* //Working solution
impl Solution {
    fn parent(vertex: &Vec<usize>, node: i32) -> usize {
        let mut parent = node as usize;
        while vertex[parent] != parent {
            parent = vertex[parent];
        }
        parent
    }

    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        //let mut redundant_edges = Vec::<Vec<i32>>::new();

        let (mut s0, mut s1) = (9999, 9999);
        let mut vertex: Vec<usize> = { 0..edges.len() + 1 }.map(|i| i).collect();

        let mut indegree: Vec<usize> = vec![9999; edges.len() + 1];

        //Find in-degree
        for (i, e) in edges.iter().enumerate() {
            if indegree[e[1] as usize] == 9999 {
                indegree[e[1] as usize] = i;
            } else {
                //Suspects
                s0 = i;
                s1 = indegree[e[1] as usize];
                break;
            }
        }

        if s0 & s1 != 9999 {
            for (i, e) in edges.iter().enumerate() {
                if i != s0 {
                    let (u, v) = (
                        Solution::parent(&vertex, e[0]),
                        Solution::parent(&vertex, e[1]),
                    );
                    if u != v {
                        vertex[v] = u;
                    } else {
                        return edges[s1].clone();
                    }
                }
            }
            return edges[s0].clone();
        }

        for (i, e) in edges.iter().enumerate() {
            let (u, v) = (
                Solution::parent(&vertex, e[0]),
                Solution::parent(&vertex, e[1]),
            );
            if u != v {
                vertex[v] = u;
            } else {
                return edges[i].clone();
            }
        }

        vec![]
        //redundant_edges.pop().unwrap_or(Vec::<i32>::new())
    }
}
/* //Refactor
 *
 *
 */
struct Solution_1;
impl Solution_1 {
    fn parent(vertex: &Vec<usize>, node: i32) -> usize {
        let mut parent = node as usize;
        while vertex[parent] != parent {
            parent = vertex[parent];
        }
        parent
    }

    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut s0, mut s1) = (9999, 9999);
        let mut vertex: Vec<usize> = { 0..edges.len() + 1 }.map(|i| i).collect();
        let mut indegree: Vec<usize> = vec![9999; edges.len() + 1];

        //Find in-degree
        for (i, e) in edges.iter().enumerate() {
            if indegree[e[1] as usize] == 9999 {
                indegree[e[1] as usize] = i;
            } else {
                (s0, s1) = (i, indegree[e[1] as usize]);
                break;
            }
        }

        for (i, e) in edges.iter().enumerate() {
            if i == s0 {
                continue;
            }
            let (u, v) = (
                Solution::parent(&vertex, e[0]),
                Solution::parent(&vertex, e[1]),
            );

            if u != v {
                vertex[v] = u;
            } else {
                if s1 != 9999 {
                    return edges[s1].clone();
                }
                return edges[i].clone();
            }
        }

        edges[s0].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Solution, Solution_1};

    fn test_cases() -> Vec<(Vec<Vec<i32>>, Vec<i32>)> {
        vec![
            (
                vec![vec![2, 1], vec![3, 1], vec![4, 2], vec![1, 4]],
                vec![2, 1],
            ),
            (vec![vec![1, 2], vec![1, 3], vec![2, 3]], vec![2, 3]),
            (
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]],
                vec![4, 1],
            ),
            (
                vec![vec![4, 2], vec![1, 5], vec![5, 2], vec![5, 3], vec![2, 4]],
                vec![4, 2],
            ),
        ]
    }

    #[test]
    fn test_find_redundant_directed_connection() {
        for (i, (input, output)) in test_cases().into_iter().enumerate() {
            println!("Testcase #{}", i + 1);
            assert_eq!(output, Solution::find_redundant_directed_connection(input));
        }
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_solution(b: &mut Bencher) {
        b.iter(|| {
            for (i, (input, output)) in test_cases().into_iter().enumerate() {
                assert_eq!(output, Solution::find_redundant_directed_connection(input));
            }
        });
    }

    #[bench]
    fn bench_solution_1(b: &mut Bencher) {
        b.iter(|| {
            for (i, (input, output)) in test_cases().into_iter().enumerate() {
                assert_eq!(
                    output,
                    Solution_1::find_redundant_directed_connection(input)
                );
            }
        });
    }
}
