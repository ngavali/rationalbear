//https://leetcode.com/problems/the-latest-time-to-catch-a-bus/
//Just find the last person onboard the last bus
struct Solution;

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort_unstable();
        passengers.sort_unstable();
        let (n_b, n_p) = (buses.len(), passengers.len());
        let mut last_bus = buses[n_b-1];
        let mut p_idx = 0;
        let mut curr_capacity = capacity;
        for b in buses {
            curr_capacity = capacity;
            while p_idx < n_p && passengers[p_idx] <= b && curr_capacity > 0
            {
                curr_capacity -= 1;
                p_idx += 1;
            }
        }
        p_idx = p_idx.saturating_sub(1);
        if curr_capacity == 0 {
            last_bus = passengers[p_idx];
        }
        while p_idx < n_p && passengers[p_idx] == last_bus {
            last_bus -= 1;
            p_idx = p_idx.wrapping_sub(1);
        }
        last_bus
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<i32>, Vec<i32>, i32, i32)> {
        vec![
            (vec![2], vec![2], 1, 1),
            (vec![10, 20], vec![2, 17, 18, 19], 2, 16),
            (vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2, 20),
            (vec![5], vec![7, 8], 1, 5),
            (vec![3], vec![2, 4], 2, 3),
            (vec![6, 8, 18, 17], vec![6, 8, 17], 1, 18),
            (vec![2, 3], vec![3], 2, 2),
            (vec![2], vec![2], 2, 1),
            (vec![5], vec![2, 3], 10000, 5),
            (vec![2, 3], vec![4, 2], 1, 3),
        ]
    }

    use super::Solution;
    #[test]
    fn test_latest_time_catch_the_bus() {
        for (buses, passengers, capacity, want) in testcases() {
            assert_eq!(
                Solution::latest_time_catch_the_bus(buses, passengers, capacity),
                want
            );
        }
    }
}
