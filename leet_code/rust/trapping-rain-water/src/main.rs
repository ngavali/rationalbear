//https://leetcode.com/problems/trapping-rain-water/

/*
 * Stack method
 * Remember its between two bounded walls and not above the given location
 */

struct Solution;

trait TrappedWaterQuantityCalculator<'a> {
    fn compute(&self) -> i32;
}
struct TrappedWaterQuantityCalculatorUnImplemented;

impl TrappedWaterQuantityCalculatorUnImplemented {
    fn new(height: &[i32]) -> Self {
        TrappedWaterQuantityCalculatorUnImplemented {}
    }
}

impl<'a> TrappedWaterQuantityCalculator<'a> for TrappedWaterQuantityCalculatorUnImplemented {
    fn compute(&self) -> i32 {
        println!("I do nothing!!!");
        0
    }
}

struct TrappedWaterQuantityCalculatorUsingStack<'a> {
    height: &'a Vec<i32>,
}

impl<'a> TrappedWaterQuantityCalculatorUsingStack<'a> {
    fn new(height: &'a Vec<i32>) -> Self {
        TrappedWaterQuantityCalculatorUsingStack { height }
    }
}
struct TrappedWaterQuantityCalculatorUsingTwoPointer<'a> {
    height: &'a Vec<i32>,
}
impl<'a> TrappedWaterQuantityCalculatorUsingTwoPointer<'a> {
    fn new(height: &'a Vec<i32>) -> Self {
        TrappedWaterQuantityCalculatorUsingTwoPointer { height }
    }
}

impl<'a> TrappedWaterQuantityCalculator<'a> for TrappedWaterQuantityCalculatorUsingStack<'a> {
    fn compute(&self) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(self.height.len());
        let mut trapped_water_quantity = 0;

        for (right_loc, &right_h) in self.height.iter().enumerate() {
            while let Some(&(_, prev_h)) = stack.last()
                && prev_h < right_h
            {
                stack.pop();
                if let Some(&(left_loc, left_h)) = stack.last() {
                    trapped_water_quantity +=
                        (left_h.min(right_h) - prev_h) * (right_loc - left_loc - 1) as i32;
                }
            }
            stack.push((right_loc, right_h));
        }
        trapped_water_quantity
    }
}

impl<'a> TrappedWaterQuantityCalculator<'a> for TrappedWaterQuantityCalculatorUsingTwoPointer<'a> {
    fn compute(&self) -> i32 {
        if self.height.len() < 2 {
            return 0;
        }
        let mut left_wall = self.height[0];
        let mut right_wall = self.height[self.height.len() - 1];
        let mut ans = 0;

        let mut left = 1;
        let mut right = self.height.len() - 2;

        while left <= right {
            match left_wall < right_wall {
                true => {
                    match left_wall > self.height[left] {
                        true => ans += left_wall - self.height[left],
                        false => left_wall = self.height[left],
                    }
                    left += 1;
                }
                false => {
                    match right_wall > self.height[right] {
                        true => ans += right_wall - self.height[right],
                        false => right_wall = self.height[right],
                    }
                    right -= 1;
                }
            }
        }
        ans
    }
}

struct TrappedWaterQuantityCalculatorStrategyBuilder {}

impl TrappedWaterQuantityCalculatorStrategyBuilder {
    fn new() -> Self {
        TrappedWaterQuantityCalculatorStrategyBuilder {}
    }
    fn build<'a>(
        &self,
        name: &'a str,
        height: &'a Vec<i32>,
    ) -> Box<dyn TrappedWaterQuantityCalculator<'a> + 'a> {
        match name {
            "Stack" => Box::new(TrappedWaterQuantityCalculatorUsingStack::new(height)),
            "TwoPointer" => Box::new(TrappedWaterQuantityCalculatorUsingTwoPointer::new(height)),
            _ => Box::new(TrappedWaterQuantityCalculatorUnImplemented::new(height)),
        }
    }
}

fn compute_trapped_water_quantity<'a>(strategy: Box<dyn TrappedWaterQuantityCalculator<'a> + 'a>) -> i32 {
    strategy.compute()
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let strategy_builder = TrappedWaterQuantityCalculatorStrategyBuilder::new();
        //let strategy = strategy_builder.build("Stack", &height);
        let trapped_water_quantity_caclulator = strategy_builder.build("TwoPointer", &height);
        compute_trapped_water_quantity(trapped_water_quantity_caclulator)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::Solution;
    fn testcases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![2, 1, 5, 3, 1, 0, 4], 9),
            (vec![4, 2, 0, 3, 2, 5], 9),
            (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
            (vec![11, 679], 0),
            (vec![766, 576, 765], 189),
            (
                vec![
                    50, 51, 52, 53, 54, 55, 58, 59, 60, 62, 63, 66, 67, 68, 69, 70, 71, 73, 74, 77,
                    80, 82, 83, 84, 86, 89, 90, 92, 94, 97, 100, 103, 104, 107, 108, 109, 110, 112,
                    113, 114, 115, 118, 121, 124, 127, 128, 131, 134, 135, 137, 138, 139, 140, 141,
                    142, 144, 146, 147, 148, 149, 152, 153, 155, 156, 157, 159, 160, 161, 163, 165,
                    167, 170, 173, 176, 177, 179, 180, 182, 184, 185, 186, 188, 190, 192, 193, 195,
                    196, 199, 201, 202, 203, 204, 206, 208, 210, 213, 216, 217, 218, 220, 222,
                ],
                0,
            ),
            (
                vec![
                    396, 392, 392, 388, 385, 383, 380, 378, 374, 372, 372, 372, 370, 367, 365, 363,
                    359, 355, 353, 352, 350, 347, 346, 345, 341, 338, 336, 336, 332, 330, 326, 324,
                    321, 318, 317, 316, 316, 314, 314, 312, 312, 309, 307, 307, 304, 302, 302, 302,
                    300, 299, 297, 296, 294, 293, 290, 286, 283, 281, 278, 276, 274, 274, 273, 273,
                    271, 271, 269, 268, 264, 260, 260, 256, 255, 254, 252, 250, 249, 246, 244, 240,
                    238, 238, 236, 233, 229, 229, 228, 226, 225, 224, 221, 219, 219, 218, 217, 216,
                    213, 210, 209, 205,
                ],
                0,
            ),
            (
                vec![
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582, 968, 582,
                    968, 582, 968, 582,
                ],
                18914,
            ),
            (
                vec![
                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                ],
                0,
            ),
        ]
    }

    #[test]
    fn test_trap() {
        for (height, want) in testcases() {
            assert_eq!(Solution::trap(height), want);
        }
    }
}
