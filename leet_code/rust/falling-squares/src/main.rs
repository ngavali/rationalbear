use std::collections::BTreeMap;

/*
https://leetcode.com/problems/falling-squares/
//Let's try to Interval Tree with BTreeMap (std::collections)
*/
const DEBUG: bool = true;

struct IntervalTree(BTreeMap<i32, BTreeMap<i32, i32>>);

impl IntervalTree {}

struct StackHeight;

impl StackHeight {
    fn from(squares: &mut [Vec<i32>]) -> Vec<i32> {
        let mut stack = Vec::with_capacity(squares.len());
        let mut height_memory = Vec::with_capacity(squares.len());
        let mut max_height = 0;

        for square in squares.iter_mut() {
            square.push(square[0] + square[1])
        }

        for i in 0..squares.len() {
            let mut current_height = 0;
            for j in 0..i {
                if DEBUG {
                    println!(
                        "i->{i} {:?} range [{} {}]",
                        squares[i],
                        squares[i][0],
                        squares[i][0] + squares[i][1]
                    );
                }
                //Check if this square sits on top of the previous one
                if squares[i][2] > squares[j][0] && squares[i][0] < squares[j][2] {
                    if DEBUG {
                        println!(
                            " j -> {j} {:?} range [{} {}]",
                            squares[j], squares[j][0], squares[j][2]
                        );
                    }
                    if current_height < height_memory[j] {
                        current_height = height_memory[j];
                    }
                }
            }
            height_memory.push(current_height + squares[i][1]);

            if max_height < *height_memory.last().unwrap() {
                max_height = *height_memory.last().unwrap();
            }

            if DEBUG {
                println!(
                    "max height -> {} current_height -> {} height_memory -> {:?} stack -> {:?}",
                    max_height,
                    current_height + squares[i][1],
                    height_memory,
                    stack
                );
            }
            stack.push(max_height);
        }

        stack
    }
}

struct SolutionNaive;

impl SolutionNaive {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        StackHeight::from(&mut positions)
    }
}

struct Solution;

impl Solution {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut bmap: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();
        let mut height_stack = Vec::with_capacity(n);

        let (mut max_height, mut current_height) = (0, 0);
        for entry in &positions {
            if DEBUG {
                println!("Searching for range {} {}", entry[0], entry[0] + entry[1]);
            }
            for (s, v) in bmap.range(..(entry[0] + entry[1])) {
                if DEBUG {
                    //println!(" Found -> s={} v={:#?}", s, v);
                }
                for (e, h) in v.range(entry[0]..) {
                    if DEBUG {
                        println!(" Found -> s={} e={} h={}", s, e, h);
                    }
                    if current_height < *h {
                        current_height = *h;
                    }
                }
            }

            current_height += entry[1];

            match bmap.get_mut(&entry[0]) {
                Some(v) => {
                    if let Some(h) = v.get_mut(&(entry[0] + entry[1])) {
                        if *h < current_height {
                            *h = current_height;
                        }
                    } else {
                        v.insert(entry[0] + entry[1] - 1, current_height);
                    }
                }
                None => {
                    let mut end_bmap = BTreeMap::new();
                    end_bmap.insert(entry[0] + entry[1] - 1, current_height);
                    bmap.insert(entry[0], end_bmap);
                }
            };

            if max_height < current_height {
                max_height = current_height;
            }

            height_stack.push(max_height);
            current_height = 0;
        }
        if DEBUG {
            println!(
                "Input {:?} BMap -> {:#?}\nStack -> {:?}",
                positions, bmap, height_stack
            );
        }
        height_stack
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 2], vec![2, 3], vec![6, 1]], vec![2, 5, 5]),
        (vec![vec![100, 100], vec![200, 100]], vec![100, 100]),
        (vec![vec![6, 1], vec![9, 2], vec![2, 4]], vec![1, 2, 4]),
        (vec![vec![9, 7], vec![1, 9], vec![3, 1]], vec![7, 16, 17]),
        (vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8]),
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        let mut bmap: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();
        let mut height_stack = Vec::new();

        let mut max_height = 0;
        for entry in test_case.0.iter() {
            let mut current_height = 0;
            println!("Searching for range {} {}", entry[0], entry[0] + entry[1]);
            for (s, v) in bmap.range(..(entry[0] + entry[1])) {
                //println!(" Found -> s={} v={:#?}", s, v);
                for (e, h) in v.range(entry[0]..) {
                    println!(" Found -> s={} e={} h={}", s, e, h);
                    if current_height < *h {
                        current_height = *h;
                    }
                }
            }

            current_height += entry[1];

            match bmap.get_mut(&entry[0]) {
                Some(v) => {
                    if let Some(h) = v.get_mut(&(entry[0] + entry[1])) {
                        if *h < entry[1] {
                            *h = entry[1];
                        }
                    } else {
                        v.insert(entry[0] + entry[1] - 1, current_height);
                    }
                }
                None => {
                    let mut end_bmap = BTreeMap::new();
                    end_bmap.insert((entry[0] + entry[1] - 1), current_height);
                    bmap.insert(entry[0], end_bmap);
                }
            };

            if max_height < current_height {
                max_height = current_height;
            }
            height_stack.push(max_height);
        }
        println!(
            "T{:02} -> {:?} BMap -> {:#?}\nStack -> {:?}",
            i, test_case.0, bmap, height_stack
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{Solution, SolutionNaive};

    #[test]
    fn test_SolutionNaive_falling_squares() {
        let test_cases = vec![
            (vec![vec![1, 2], vec![2, 3], vec![6, 1]], vec![2, 5, 5]),
            (vec![vec![100, 100], vec![200, 100]], vec![100, 100]),
            (vec![vec![6, 1], vec![9, 2], vec![2, 4]], vec![1, 2, 4]),
            (vec![vec![9, 7], vec![1, 9], vec![3, 1]], vec![7, 16, 17]),
            (vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8]),
            (
                vec![vec![4, 1], vec![6, 9], vec![6, 8], vec![1, 9], vec![9, 8]],
                vec![1, 9, 17, 26, 34],
            ),
        ];

        for test_case in test_cases {
            assert_eq!(test_case.1, SolutionNaive::falling_squares(test_case.0));
        }
    }

    #[test]
    fn test_Solution_falling_squares() {
        let test_cases = vec![
            (vec![vec![1, 2], vec![2, 3], vec![6, 1]], vec![2, 5, 5]),
            (vec![vec![100, 100], vec![200, 100]], vec![100, 100]),
            (vec![vec![6, 1], vec![9, 2], vec![2, 4]], vec![1, 2, 4]),
            (vec![vec![9, 7], vec![1, 9], vec![3, 1]], vec![7, 16, 17]),
            (vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8]),
            (
                vec![vec![4, 1], vec![6, 9], vec![6, 8], vec![1, 9], vec![9, 8]],
                vec![1, 9, 17, 26, 34],
            ),
        ];

        for test_case in test_cases {
            assert_eq!(test_case.1, Solution::falling_squares(test_case.0));
        }
    }
}
