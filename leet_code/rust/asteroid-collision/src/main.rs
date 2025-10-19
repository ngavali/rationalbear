//https://leetcode.com/problems/asteroid-collision/
struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut left_moving: Vec<i32> = Vec::new();
        let mut right_moving: Vec<i32> = Vec::new();
        for asteroid in asteroids {
            if asteroid > 0 {
                right_moving.push(asteroid);
            } else {
                left_moving.push(asteroid);
                while !right_moving.is_empty() && !left_moving.is_empty() {
                    match right_moving[right_moving.len() - 1]
                        .abs()
                        .cmp(&left_moving[left_moving.len() - 1].abs())
                    {
                        Ordering::Less => {
                            right_moving.pop();
                        }
                        Ordering::Greater => {
                            left_moving.pop();
                            break;
                        }
                        Ordering::Equal => {
                            right_moving.pop();
                            left_moving.pop();
                            break;
                        }
                    }
                }
            }
        }
        left_moving.extend(right_moving);
        left_moving
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, Vec<i32>)> {
        vec![
            (vec![5, 10, -5], vec![5, 10]),
            (vec![8, -8], vec![]),
            (vec![10, 2, -5], vec![10]),
            (vec![3, 5, -6, 2, -1, 4], vec![-6, 2, 4]),
        ]
    }

    use super::Solution;
    #[test]
    fn test_asteroid_collision() {
        for (asteroids, want) in testcases() {
            assert_eq!(Solution::asteroid_collision(asteroids), want);
        }
    }
}
