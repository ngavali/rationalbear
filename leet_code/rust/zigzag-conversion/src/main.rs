//https://leetcode.com/problems/zigzag-conversion/description/
struct Solution {}

impl Solution {
    fn fill_top_or_bottom(s: &Vec<u8>, rows: usize, mut index: usize, res: &mut Vec<u8>) {
        let large_step = 2 * (rows - 1);
        let n = s.len();
        for index in (index..n).step_by(large_step) {
            res.push(s[index]);
        }
    }

    fn fill_middle(s: &Vec<u8>, rows: usize, mut index: usize, res: &mut Vec<u8>) {
        let (small_step, large_step) = (2 * (rows - 1 - index), 2 * (rows - 1));
        let n = s.len();
        let mut next_index = 0;
        for index in (index..n).step_by(large_step) {
            next_index = index + small_step;
            res.push(s[index]);
            if next_index < n {
                res.push(s[next_index]);
            }
        }
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows > 1 && s.len() > 1 {
            let s = s.as_bytes().to_vec();
            let mut res = Vec::<u8>::new();
            Solution::fill_top_or_bottom(&s, num_rows as usize, 0, &mut res);
            for row in 1..(num_rows - 1) {
                Solution::fill_middle(&s, num_rows as usize, row as usize, &mut res);
            }
            Solution::fill_top_or_bottom(&s, num_rows as usize, (num_rows - 1) as usize, &mut res);
            return String::from_utf8(res).unwrap();
        }
        s
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_convert() {
        let testcases = vec![
            (
                String::from("PAYPALISHIRING"),
                2,
                String::from("PYAIHRNAPLSIIG"),
            ),
            (
                String::from("PAYPALISHIRING"),
                3,
                String::from("PAHNAPLSIIGYIR"),
            ),
            (
                String::from("PAYPALISHIRING"),
                4,
                String::from("PINALSIGYAHRPI"),
            ),
            (String::from("A"), 1, String::from("A")),
            (String::from("AB"), 1, String::from("AB")),
        ];

        for testcase in testcases.into_iter().take(1) {
            assert_eq!(testcase.2, Solution::convert(testcase.0, testcase.1));
        }
    }
}
