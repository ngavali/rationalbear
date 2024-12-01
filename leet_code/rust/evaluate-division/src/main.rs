/* Medium
 * https://leetcode.com/problems/evaluate-division/
 */

struct Solution;
use std::collections::{HashMap, HashSet};

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
}

#[cfg(test)]

mod tests {
    use crate::Solution;

    #[test]
    fn test_calc_equation() {
        let test_cases = vec![
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
        ];

        for (equations, values, queries, output) in test_cases {
            assert_eq!(output, Solution::calc_equation(equations, values, queries));
        }
    }
}
