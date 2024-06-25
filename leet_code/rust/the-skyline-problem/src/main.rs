use std::cmp::Ordering;

struct Solution {}

struct Point {
    x: i32,
    h: i32,
}

struct PriorityQueue(Vec<i32>);

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue(Vec::new())
    }
    fn add(&mut self, v: i32) {
        self.0.push(v);
    }

    fn remove(&mut self, v: i32) {
        match self.0.iter().position(|&value| value == v) {
            Some(i) => {
                self.0.remove(i);
            }
            None => {}
        }
    }
    fn get_top(&mut self) -> i32 {
        self.0.sort();
        match self.0.last() {
            Some(&v) => v,
            None => 0,
        }
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points: Vec<Point> = Vec::new();

        let mut final_points = Vec::new();

        for building in buildings {
            points.push(Point {
                x: building[0],
                h: -building[2],
            });
            points.push(Point {
                x: building[1],
                h: building[2],
            });
        }

        points.sort_by(|p1, p2| match p1.x.cmp(&p2.x) {
            Ordering::Equal => p1.h.cmp(&p2.h),
            whatever => whatever,
        });

        let mut max_heap = PriorityQueue::new();

        let mut previous_max_height = 0;
        for point in points {
            if point.h < 0 {
                max_heap.add(point.h.abs());
            } else {
                max_heap.remove(point.h);
            }
            let current_max_height = max_heap.get_top();
            if current_max_height != previous_max_height {
                final_points.push(vec![point.x, current_max_height]);
            }
            previous_max_height = current_max_height;
        }
        final_points
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8],
            ],
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0],
            ],
        ),
        (
            vec![vec![0, 2, 3], vec![2, 5, 3]],
            vec![vec![0, 3], vec![5, 0]],
        ),
    ];

    for (input, _) in test_cases {
        println!("{:#?}", Solution::get_skyline(input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_skyline() {
        let test_cases = vec![
            (
                vec![
                    vec![2, 9, 10],
                    vec![3, 7, 15],
                    vec![5, 12, 12],
                    vec![15, 20, 10],
                    vec![19, 24, 8],
                ],
                vec![
                    vec![2, 10],
                    vec![3, 15],
                    vec![7, 12],
                    vec![12, 0],
                    vec![15, 10],
                    vec![20, 8],
                    vec![24, 0],
                ],
            ),
            (
                vec![vec![0, 2, 3], vec![2, 5, 3]],
                vec![vec![0, 3], vec![5, 0]],
            ),
        ];

        for (input, exp_output) in test_cases {
            let output = Solution::get_skyline(input);
            assert_eq!(exp_output, output);
        }
    }
}
