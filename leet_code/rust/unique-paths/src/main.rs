//https://leetcode.com/problems/unique-paths/
struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m,n) = (m as usize,n as usize);
        let mut prev = vec![1; n];
        let mut next = vec![1; n];
        for x in 0..m-1 {
            for y in 0..n-1 {
                next[y+1] = prev[y+1] + next[y];
            }
            std::mem::swap(&mut prev, &mut  next);
        }
        prev[n-1]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    fn testcases() -> Vec<(i32, i32, i32)> {
        vec![
            (3,7,28),
            (3,2,3),
            (10,10,48620),
            (10,5,715),
            (1,2,1),
            (23,12,193536720),
        ]
    }

    use super::Solution;
    #[test]
    fn test_unique_paths() {
        for (m,n,want) in testcases() {
            assert_eq!(Solution::unique_paths(m,n), want);
        }
    }
}
