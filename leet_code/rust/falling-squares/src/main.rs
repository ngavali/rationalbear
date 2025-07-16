/*
https://leetcode.com/problems/falling-squares/
//Let's try to Interval Tree with BTreeMap (std::collections)
*/
struct SegmentTree(Vec<(i32, i32)>);

impl SegmentTree {
    fn with_capacity(leaves: usize) -> Self {
        SegmentTree(vec![(0, 0); leaves << 1])
    }

    fn update(&mut self, l: usize, h: usize, s: usize, e: usize, pos: usize, val: i32) {
        if s >= e {
            //|| s >= h || e <= l || s == e || l == h || s >= e {
            return;
        }

        if (s ^ l ^ e ^ h) == 0 {
            self.0[pos].0 = self.0[pos].0.max(val);
            return;
        }

        //Observed max
        self.0[pos].1 = self.0[pos].1.max(val);
        let partition = (h + l) >> 1;
        self.update(l, partition, s, e.min(partition), (pos << 1), val);
        self.update(partition, h, s.max(partition), e, (pos << 1) + 1, val);
    }

    fn fetch(&self, l: usize, h: usize, s: usize, e: usize, pos: usize) -> i32 {
        if s >= e {
            //|| s >= h || e <= l || s == e || l == h || s >= e {
            return 0;
        }

        if (s ^ l ^ e ^ h) == 0 {
            return self.0[pos].0.max(self.0[pos].1);
        }

        let partition = h - (h - l) / 2;
        let l_val = self.fetch(l, partition, s, e.min(partition), (pos << 1));
        let r_val = self.fetch(partition, h, s.max(partition), e, (pos << 1) + 1);
        self.0[pos].0.max(l_val).max(r_val)
    }
}

struct Solution;

impl Solution {
    pub fn falling_squares(mut positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut result: Vec<i32> = Vec::with_capacity(n);
        let mut corners: Vec<i32> = Vec::with_capacity(n);

        for input in positions.iter() {
            corners.push(input[0]);
            corners.push(input[0] + input[1]);
        }

        corners.sort_unstable();
        corners.dedup();

        let mut leaves = 1; //= 2usize.pow((corners.len() * 2).ilog2());
        while leaves < corners.len() {
            leaves <<= 1;
        }
        let mut range_data = SegmentTree::with_capacity(leaves);

        let (mut current_height, mut max_height) = (0, 0);
        let (mut s, mut e) = (0, 0);
        for input in &positions {
            s = corners.binary_search(&input[0]).unwrap();
            e = corners.binary_search(&(input[0] + input[1])).unwrap();
            current_height = range_data.fetch(0, leaves, s, e, 1) + input[1];
            range_data.update(0, leaves, s, e, 1, current_height);
            result.push(range_data.fetch(0, leaves, 0, leaves, 1));
        }

        result
    }
}

fn test_cases() -> Vec<(Vec<Vec<i32>>, Vec<i32>)> {
    vec![
        (vec![vec![1, 2], vec![2, 3], vec![6, 1]], vec![2, 5, 5]),
        (vec![vec![100, 100], vec![200, 100]], vec![100, 100]),
        (vec![vec![6, 1], vec![9, 2], vec![2, 4]], vec![1, 2, 4]),
        (vec![vec![9, 7], vec![1, 9], vec![3, 1]], vec![7, 16, 17]),
        (vec![vec![9, 6], vec![2, 2], vec![2, 6]], vec![6, 6, 8]),
        (
            vec![vec![4, 1], vec![6, 9], vec![6, 8], vec![1, 9], vec![9, 8]],
            vec![1, 9, 17, 26, 34],
        ),
    ]
}

fn main() {
    for (i, test_case) in test_cases().iter().enumerate() {
        if i == 5 {
            println!(
                "T{:02} {:?}\nresult={:?}",
                i,
                test_case.0,
                Solution::falling_squares(test_case.0.clone())
            );
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{test_cases, Solution};

    #[test]
    fn test_Solution_falling_squares() {
        for test_case in test_cases() {
            assert_eq!(test_case.1, Solution::falling_squares(test_case.0));
        }
    }
}
