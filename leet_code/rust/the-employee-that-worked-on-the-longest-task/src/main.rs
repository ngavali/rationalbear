struct Solution;
//https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut ans = (logs[0][1], logs[0][0]);
        for i in 1..logs.len() {
            let time_spent = logs[i][1] - logs[i-1][1];
            if ans.0 < time_spent {
                ans = (time_spent, logs[i][0]);
            } else if ans.0 == time_spent {
                if ans.1 > logs[i][0] {
                    ans.1 = logs[i][0];
                }
            }
        }
        ans.1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, Vec<Vec<i32>>, i32)> {
        vec![
            (10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]], 1),
            (
                26,
                vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]],
                3,
            ),
            (2, vec![vec![0, 10], vec![1, 20]], 0),
        ]
    }

    use super::Solution;
    #[test]
    fn test_harderst_worker() {
        for (n, logs, want) in testcases() {
            assert_eq!(Solution::hardest_worker(n, logs), want);
        }
    }
}

/* Pass1
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut ans = (0,n);
        let mut curr_time = 0;
        logs.iter().for_each(| log | {
            let time_spent = log[1] - curr_time;
            if ans.0 < time_spent {
                ans = ( time_spent, log[0] );
            } else if ans.0 == time_spent {
                if ans.1 > log[0] {
                    ans.1 = log[0];
                }
            }
            curr_time = log[1];
        });
        ans.1
    }
}
*/
