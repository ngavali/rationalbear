//https://leetcode.com/problems/merge-intervals/
struct Solution;
impl Solution {
    pub fn merge_in_place(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|i1, i2| i1[0].cmp(&i2[0]));
        let mut left = 0;
        let mut right = 1;
        while right < intervals.len() {
            if intervals[right][0] <= intervals[left][1] {
                intervals[left][1] = intervals[left][1].max(intervals[right][1]);
            } else {
                left += 1;
                intervals[left] = intervals[right].clone();
            }
            right += 1;
        }
        intervals.truncate(left+1);
        intervals
    }
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut intervals = intervals;
        intervals.sort_by(|i1, i2| i2[0].cmp(&i1[0]));
        if let Some(interval) = intervals.pop() {
            ans.push(interval);
        }
        while let Some(next_interval) = intervals.pop() {
            if let Some(prev_interval) = ans.last_mut() {
                if next_interval[0] <= prev_interval[1] {
                    prev_interval[1] = next_interval[1].max(prev_interval[1]);
                } else {
                    ans.push(next_interval);
                }
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> {
        vec![
            (
                vec![
                    vec![1, 1],
                    vec![2, 2],
                    vec![0, 0],
                    vec![2, 3],
                    vec![1, 3],
                    vec![3, 5],
                    vec![2, 3],
                    vec![3, 5],
                ],
                vec![vec![0, 0], vec![1, 5]],
            ),
            (
                vec![
                    vec![2, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![1, 3],
                    vec![5, 7],
                    vec![2, 2],
                    vec![4, 6],
                ],
                vec![vec![1, 3], vec![4, 7]],
            ),
            (
                vec![
                    vec![1, 3],
                    vec![2, 6],
                    vec![8, 10],
                    vec![8, 9],
                    vec![9, 11],
                    vec![15, 18],
                    vec![2, 4],
                    vec![16, 17],
                ],
                vec![vec![1, 6], vec![8, 11], vec![15, 18]],
            ),
            (
                vec![vec![1012, 1136], vec![1137, 1417], vec![1015, 1020]],
                vec![vec![1012, 1136], vec![1137, 1417]],
            ),
            (vec![vec![1, 3]], vec![vec![1, 3]]),
            (vec![vec![1, 4], vec![2, 3]], vec![vec![1, 4]]),
            (
                vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]],
                vec![vec![1, 10]],
            ),
            (vec![vec![4, 7], vec![1, 4]], vec![vec![1, 7]]),
            (
                vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
                vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            ),
            (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
        ]
    }
    use super::Solution;

    #[test]
    fn test_merge() {
        for (intervals, mut want) in testcases() {
            let mut got = Solution::merge(intervals);
            got.sort();
            want.sort();
            assert_eq!(got, want);
        }
    }
    #[test]
    fn test_merge_in_place() {
        for (intervals, mut want) in testcases() {
            let mut got = Solution::merge_in_place(intervals);
            got.sort();
            want.sort();
            assert_eq!(got, want);
        }
    }
}
