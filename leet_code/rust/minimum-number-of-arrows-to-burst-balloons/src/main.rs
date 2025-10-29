//https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a, b| a.cmp(&b));
        let mut c = 1;
        let mut c_r = (points[0][0], points[0][1]);
        points[1..].iter().for_each(|p| {
            if c_r.1 < p[0] {
                c_r = (p[0], p[1]);
                c += 1;
            } else {
                c_r.1 = c_r.1.min(p[1]);
                c_r.0 = c_r.0.max(p[0]);
            }
        });
        c
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]], 2),
            (vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 4),
            (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], 2),
            (
                vec![
                    vec![9, 12],
                    vec![1, 10],
                    vec![4, 11],
                    vec![8, 12],
                    vec![3, 9],
                    vec![6, 9],
                    vec![6, 7],
                ],
                2,
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_find_min_arrow_shots() {
        for (points, want) in testcases() {
            assert_eq!(Solution::find_min_arrow_shots(points), want);
        }
    }
}
