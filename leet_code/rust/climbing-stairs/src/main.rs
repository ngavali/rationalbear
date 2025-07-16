/* Easy
 * https://leetcode.com/problems/climbing-stairs
 *
 */
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut n = n;
        let (mut i, mut j) = (0, 1);
        while n >= 0 {
            (j, i) = (j + i, j);
            n -= 1;
        }
        i
    }
}

fn main() {
    println!("Hello, world!");
}

fn testcases() -> Vec<(i32, i32)> {
    vec![(2, 2), (3, 3)]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_climb_stairs() {
        for (i, (n, o)) in testcases().into_iter().enumerate() {
            println!("Testcase#{i}");
            assert_eq!(o, Solution::climb_stairs(n));
        }
    }
}
