use std::cmp::Ordering;

struct Corner {
    x: i32,
    h: i32,
}

struct SkyLine {}

impl SkyLine {
    fn find(corners: &Vec<Corner>) -> Vec<Vec<i32>> {
        let mut skyline = Vec::with_capacity(corners.len());
        let mut max_heap = MaxHeap::with_capacity(corners.len() / 2);

        let (mut current_max_height, mut previous_max_height) = (0, 0);
        for point in corners {
            if point.h < 0 {
                max_heap.add(-point.h);
            } else {
                max_heap.remove(point.h);
            }
            current_max_height = max_heap.get();
            if current_max_height != previous_max_height {
                skyline.push(vec![point.x, current_max_height]);
                previous_max_height = current_max_height;
            }
        }
        skyline
    }
}

impl Corner {
    fn from(buildings: Vec<Vec<i32>>) -> Vec<Corner> {
        let mut corners: Vec<Corner> = Vec::with_capacity(buildings.len());

        for building in buildings {
            corners.push(Corner {
                x: building[0],
                h: -building[2],
            });
            corners.push(Corner {
                x: building[1],
                h: building[2],
            });
        }

        corners.sort_by(|p1, p2| match p1.x.cmp(&p2.x) {
            Ordering::Equal => p1.h.cmp(&p2.h),
            whatever => whatever,
        });

        corners
    }
}

struct MaxHeap(Vec<i32>);

impl MaxHeap {
    fn with_capacity(size: usize) -> Self {
        MaxHeap(Vec::with_capacity(size))
    }
    fn add(&mut self, v: i32) {
        self.0.push(v);
        self.0.sort();
    }

    fn remove(&mut self, v: i32) {
        match self.0.iter().position(|&value| value == v) {
            Some(i) => {
                self.0.remove(i);
            }
            None => {}
        }
    }
    fn get(&mut self) -> i32 {
        match self.0.last() {
            Some(&v) => v,
            None => 0,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let N = buildings.len();

        let mut corners: Vec<Corner> = Corner::from(buildings);

        SkyLine::find(&corners)
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
