//https://leetcode.com/problems/find-closest-person/description/
struct Solution {}
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let (dx, dy) = (z.abs_diff(x), z.abs_diff(y));
        (!(dx == dy) as i32) * ((dx > dy) as i32 + 1)
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_closest() {
        let testcases = vec![(2, 7, 4, 1), (2, 5, 6, 2), (1, 5, 3, 0)];
        for (x, y, z, want) in testcases {
            assert_eq!(Solution::find_closest(x,y,z), want);
        }
    }
}
