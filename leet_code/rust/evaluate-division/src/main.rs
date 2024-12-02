#![feature(test)]
/* Medium
 * https://leetcode.com/problems/evaluate-division/
 */

struct Solution;
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph_map: HashMap<String, HashMap<String, f64>> =
            HashMap::with_capacity(equations.len());

        equations.iter().enumerate().for_each(|(i, eq)| {
            graph_map
                .entry(eq[0].clone())
                .or_insert(HashMap::new())
                .insert(eq[1].clone(), values[i]);
            graph_map
                .entry(eq[1].clone())
                .or_insert(HashMap::new())
                .insert(eq[0].clone(), 1.0 / values[i]);
        });

        queries
            .into_iter()
            .map(|query| {
                let mut visited: HashSet<&String> = HashSet::with_capacity(equations.len() * 2);
                let mut ans = -1.0;
                match graph_map.contains_key(&query[0]) {
                    true => {
                        Solution::dfs(
                            &graph_map,
                            &query[0],
                            &query[1],
                            &mut ans,
                            1.0,
                            &mut visited,
                        );
                        ans
                    }
                    false => -1.0,
                }
            })
            .collect()
    }

    fn dfs<'a>(
        graph_map: &'a HashMap<String, HashMap<String, f64>>,
        start_key: &'a String,
        target_key: &String,
        final_ans: &mut f64,
        ans: f64,
        visited: &mut HashSet<&'a String>,
    ) {
        if visited.contains(start_key) {
            return;
        }
        if start_key == target_key {
            *final_ans = ans;
            return;
        }
        visited.insert(start_key);
        if let Some(list) = graph_map.get(start_key) {
            for (next_key, value) in list.iter() {
                Solution::dfs(
                    graph_map,
                    next_key,
                    target_key,
                    final_ans,
                    ans * *value,
                    visited,
                );
            }
        }
    }

    pub fn calc_equation_bfs(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph_map: HashMap<String, HashMap<String, f64>> =
            HashMap::with_capacity(equations.len());

        equations.iter().enumerate().for_each(|(i, eq)| {
            graph_map
                .entry(eq[0].clone())
                .or_insert(HashMap::new())
                .insert(eq[1].clone(), values[i]);
            graph_map
                .entry(eq[1].clone())
                .or_insert(HashMap::new())
                .insert(eq[0].clone(), 1.0 / values[i]);
        });

        queries
            .into_iter()
            .map(|query| Solution::bfs(&graph_map, &query[0], &query[1]))
            .collect()
    }

    fn bfs<'a>(
        graph_map: &'a HashMap<String, HashMap<String, f64>>,
        start_key: &'a String,
        target_key: &String,
    ) -> f64 {
        if !graph_map.contains_key(start_key) || !graph_map.contains_key(target_key) {
            return -1.0;
        }
        let mut queue = VecDeque::new();
        queue.push_back((start_key, 1.0));
        let mut visited = HashSet::new();

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some((start_key, v)) = queue.pop_front() {
                    //visited.insert(start_key);    //~7,217.12 ns/iter (+/- 272.79)
                    if start_key == target_key {
                        return v;
                    }
                    if let Some(list) = graph_map.get(start_key) {
                        for (next_key, nv) in list.iter() {
                            if !visited.contains(next_key) {
                                queue.push_back((next_key, v * nv));
                            }
                        }
                    }
                    visited.insert(start_key); //~6,764.25 ns/iter (+/- 173.53)
                }
            }
        }
        -1.0
    }

    pub fn calc_equation_other(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        use std::collections::HashMap;

        fn dfs_other(
            graph: &HashMap<String, HashMap<String, f64>>,
            visited: &mut HashMap<String, bool>,
            start: &str,
            end: &str,
            value: f64,
        ) -> f64 {
            if !graph.contains_key(start) || !graph.contains_key(end) {
                return -1.0;
            }
            if start == end {
                return value;
            }

            visited.insert(start.to_string(), true);
            if let Some(neighbors) = graph.get(start) {
                for (next_node, &weight) in neighbors {
                    if !visited.contains_key(next_node) {
                        let result = dfs_other(graph, visited, next_node, end, value * weight);
                        if result != -1.0 {
                            return result;
                        }
                    }
                }
            }
            -1.0
        }

        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();

        // Build the graph
        for i in 0..equations.len() {
            let a = &equations[i][0];
            let b = &equations[i][1];
            let value = values[i];

            graph
                .entry(a.clone())
                .or_insert_with(HashMap::new)
                .insert(b.clone(), value);
            graph
                .entry(b.clone())
                .or_insert_with(HashMap::new)
                .insert(a.clone(), 1.0 / value);
        }

        // Answer each query
        let mut result = Vec::new();
        for query in queries {
            let c = &query[0];
            let d = &query[1];
            let mut visited = HashMap::new();
            let res = dfs_other(&graph, &mut visited, c, d, 1.0);
            result.push(res);
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::calc_equation(
            vec![
                vec![String::from("a"), String::from("b")],
                vec![String::from("c"), String::from("d")],
            ],
            vec![1.0, 1.0],
            vec![
                vec![String::from("a"), String::from("c")],
                vec![String::from("b"), String::from("d")],
                vec![String::from("b"), String::from("a")],
                vec![String::from("d"), String::from("c")],
            ]
        )
    );
}

#[cfg(test)]

mod tests {
    use crate::Solution;

    fn test_cases() -> Vec<(Vec<Vec<String>>, Vec<f64>, Vec<Vec<String>>, Vec<f64>)> {
        vec![
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("c"), String::from("d")],
                ],
                vec![1.0, 1.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("d")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("d"), String::from("c")],
                ],
                vec![-1.00000, -1.00000, 1.00000, 1.00000],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")],
                ],
                vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            ),
            (
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("bc"), String::from("cd")],
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("c"), String::from("b")],
                    vec![String::from("bc"), String::from("cd")],
                    vec![String::from("cd"), String::from("bc")],
                ],
                vec![3.75000, 0.40000, 5.00000, 0.20000],
            ),
            (
                vec![vec![String::from("a"), String::from("b")]],
                vec![0.5],
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("x"), String::from("y")],
                ],
                vec![0.50000, 2.00000, -1.00000, -1.00000],
            ),
        ]
    }

    #[test]
    fn test_calc_equation() {
        let test_cases = test_cases();
        for (equations, values, queries, output) in test_cases {
            assert_eq!(output, Solution::calc_equation(equations, values, queries));
        }
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_mysolution(b: &mut Bencher) {
        b.iter(|| {
            for (equations, values, queries, output) in test_cases() {
                assert_eq!(output, Solution::calc_equation(equations, values, queries));
            }
        });
    }
    #[bench]
    fn bench_mysolution_bfs(b: &mut Bencher) {
        b.iter(|| {
            for (equations, values, queries, output) in test_cases() {
                assert_eq!(
                    output,
                    Solution::calc_equation_bfs(equations, values, queries)
                );
            }
        });
    }
    #[bench]
    fn bench_othersolution(b: &mut Bencher) {
        b.iter(|| {
            for (equations, values, queries, output) in test_cases() {
                assert_eq!(
                    output,
                    Solution::calc_equation_other(equations, values, queries)
                );
            }
        });
    }
}

/* Benchmarks
 *
 * $ rustup run nightly cargo bench
 * Finished `bench` profile [optimized] target(s) in 0.00s
 * Running unittests src/main.rs (target/release/deps/evaluate_division-d00a26a6c189747e)
 * running 3 tests
 * test tests::test_calc_equation ... ignored
 * test tests::bench_mysolution    ... bench:       7,331.27 ns/iter (+/- 108.74)
 * test tests::bench_othersolution ... bench:       7,838.72 ns/iter (+/- 157.56)
 *
 * test result: ok. 0 passed; 0 failed; 1 ignored; 2 measured; 0 filtered out; finished in 8.44s
 *
 */
