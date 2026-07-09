struct Solution {}

#[derive(Debug)]
struct UnionSet {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl UnionSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (root_x, root_y) = (self.find(x), self.find(y));

        if root_x != root_y {
            if self.size[root_x] >= self.size[root_y] {
                self.parent[root_y] = root_x;
                self.size[root_x] += 1;
            } else {
                self.parent[root_x] = root_y;
                self.size[root_y] += 1;
            }
        }
    }
}

use std::collections::VecDeque;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut thief_cells = VecDeque::with_capacity(m * n);
        let mut thief_distance = vec![vec![2000; n]; m];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    thief_cells.push_back((i, j));
                    thief_distance[i][j] = 0;
                }
            }
        }

        let mut bfs_q = thief_cells;
        let mut cells_with_distance: Vec<(i32, (usize, usize))> = Vec::with_capacity(m * n);
        while let Some(cell) = bfs_q.pop_front() {
            cells_with_distance.push((thief_distance[cell.0][cell.1], cell));
            if cell.0 + 1 < m && thief_distance[cell.0 + 1][cell.1] == 2000 {
                bfs_q.push_back((cell.0 + 1, cell.1));
                thief_distance[cell.0 + 1][cell.1] = thief_distance[cell.0][cell.1] + 1;
            }
            if cell.1 + 1 < n && thief_distance[cell.0][cell.1 + 1] == 2000 {
                bfs_q.push_back((cell.0, cell.1 + 1));
                thief_distance[cell.0][cell.1 + 1] = thief_distance[cell.0][cell.1] + 1;
            }
            if cell.0 > 0 && thief_distance[cell.0 - 1][cell.1] == 2000 {
                bfs_q.push_back((cell.0 - 1, cell.1));
                thief_distance[cell.0 - 1][cell.1] = thief_distance[cell.0][cell.1] + 1;
            }
            if cell.1 > 0 && thief_distance[cell.0][cell.1 - 1] == 2000 {
                bfs_q.push_back((cell.0, cell.1 - 1));
                thief_distance[cell.0][cell.1 - 1] = thief_distance[cell.0][cell.1] + 1;
            }
        }

        println!("thief_distance -> {thief_distance:?}");
        println!("cells_with_distance -> {cells_with_distance:?}");

        let mut us = UnionSet::new(m * n);
        println!("Union Set = {us:?}");
        for (distance, cell) in cells_with_distance.into_iter().rev() {
            let idx = n * cell.0 + cell.1;
            println!("{distance} {cell:?}");
            if (cell.0 > 0 && thief_distance[cell.0 - 1][cell.1] >= distance) {
                let neighbor_idx = n * (cell.0 - 1) + cell.1;
                us.union(idx, neighbor_idx);
                println!("up:    Union Set({idx}, {neighbor_idx}) = {us:?}");
            }
            if (cell.1 > 0 && thief_distance[cell.0][cell.1 - 1] >= distance) {
                let neighbor_idx = n * cell.0 + (cell.1 - 1);
                us.union(idx, neighbor_idx);
                println!("left:  Union Set({idx}, {neighbor_idx}) = {us:?}");
            }
            if (cell.1 + 1 < n && thief_distance[cell.0][cell.1 + 1] >= distance) {
                let neighbor_idx = n * cell.0 + cell.1 + 1;
                us.union(n * cell.0 + cell.1, neighbor_idx);
                println!("right: Union Set({idx}, {neighbor_idx}) = {us:?}");
            }
            if (cell.0 + 1 < m && thief_distance[cell.0 + 1][cell.1] >= distance) {
                let neighbor_idx = n * (cell.0 + 1) + cell.1;
                us.union(idx, neighbor_idx);
                println!("down:  Union Set({idx}, {neighbor_idx}) = {us:?}");
            }
            if us.find(0) == us.find(m * n - 1) {
                return distance;
            }
        }
        0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(Vec<Vec<i32>>, i32)> {
        vec![
            (vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]], 0),
            (vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]], 2),
            (
                vec![
                    vec![0, 0, 0, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![1, 0, 0, 0],
                ],
                2,
            ),
        ]
    }

    #[test]
    fn test_calculate_minimum_hp() {
        for (i, (grid, want)) in testcases().into_iter().enumerate() {
            println!("---- Testcase #{i} ----");
            let got = Solution::maximum_safeness_factor(grid);
            println!("Want -> {want:?}");
            println!("Got  -> {got:?}");
            assert_eq!(want, got);
        }
    }
}
