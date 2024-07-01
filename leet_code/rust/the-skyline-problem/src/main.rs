use std::{cmp::Ordering, collections::BTreeMap};

struct MaxHeap(BTreeMap<i32, u64>);

impl MaxHeap {
    fn new() -> Self {
        MaxHeap(BTreeMap::new())
    }

    fn add(&mut self, k: i32) {
        match self.0.get_mut(&k) {
            Some(v) => *v += 1,
            None => {
                self.0.insert(k, 0);
            }
        };
    }

    fn remove(&mut self, k: i32) {
        match self.0.get_mut(&k) {
            Some(v) if *v >= 1 => {
                *v -= 1;
            }
            _ => {
                self.0.remove(&k);
            }
        };
    }
    fn get(&mut self) -> i32 {
        match self.0.last_key_value() {
            Some((&k, _)) => k,
            None => 0,
        }
    }
}

struct Corner {
    x: i32,
    h: i32,
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

struct SkyLine {}

impl SkyLine {
    fn from(corners: &Vec<Corner>) -> Vec<Vec<i32>> {
        let mut skyline = Vec::with_capacity(corners.len());
        let mut max_heap = MaxHeap::new();

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

struct Solution {}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let corners: Vec<Corner> = Corner::from(buildings);

        SkyLine::from(&corners)
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
