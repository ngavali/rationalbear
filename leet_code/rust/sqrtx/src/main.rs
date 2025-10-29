//https://leetcode.com/problems/sqrtx/
struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u64;
        let mut l = 1;
        let mut r = x;
        while l < r {
            let mid = (l + r) / 2;
            if l == mid {
                return l as i32;
            }
            if mid * mid < x {
                l = mid;
            } else if mid * mid > x {
                r = mid;
            } else {
                return mid as i32;
            }
        }
        ((l + r) / 2) as i32
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(i32, i32)> {
        vec![
            (1, 1),
            (0, 0),
            (8, 2),
            (10, 3),
            (2000200, 1414),
            (100000, 316),
            (1020202, 1010),
            (2147395599, 46339),
        ]
    }

    use super::Solution;
    #[test]
    fn test_my_sqrt() {
        for (x, want) in testcases() {
            assert_eq!(Solution::my_sqrt(x), want);
        }
    }
}
