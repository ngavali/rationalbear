//https://leetcode.com/problems/restore-ip-addresses/
struct Solution;

impl Solution {
    fn is_valid_num_str(s: &str) -> bool {
        if s.len() < 4 {
            let num: i32 = s.parse::<i32>().unwrap();
            let num_str: String = num.to_string();
            return num < 256 && s.len() == num_str.len();
        }
        false
    }
    fn is_valid_num(s: &String) -> bool {
        if s.len() < 4 {
            let num: i32 = s.parse::<i32>().unwrap();
            let num_str: String = num.to_string();
            return num < 256 && s.len() == num_str.len();
        }
        false
    }
    fn bt(s: String, mut left: Vec<String>, dot: u8, solutions: &mut Vec<String>) {
        if dot == 3 {
            if Self::is_valid_num(&s) {
                left.push(s.to_string());
                solutions.push(left.join("."));
            }
            return;
        }
        for idx in 1..(s.len().min(4)) {
            let (l, r) = s.split_at(idx);
            if Self::is_valid_num_str(l) {
                left.push(l.to_string());
                Self::bt(String::from(r), left.clone(), dot + 1, solutions);
                left.pop();
            }
        }
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut solutions = Vec::new();
        Self::bt(s, Vec::new(), 0, &mut solutions);
        solutions
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_restore_ip_addresses() {
        let testcases = vec![
            (
                String::from("25525511135"),
                vec![
                    String::from("255.255.11.135"),
                    String::from("255.255.111.35"),
                ],
            ),
            (String::from("0000"), vec![String::from("0.0.0.0")]),
            (
                String::from("101023"),
                vec![
                    String::from("1.0.10.23"),
                    String::from("1.0.102.3"),
                    String::from("10.1.0.23"),
                    String::from("10.10.2.3"),
                    String::from("101.0.2.3"),
                ],
            ),
            (
                "172162541".to_string(),
                vec![
                    "17.216.25.41".to_string(),
                    "17.216.254.1".to_string(),
                    "172.16.25.41".to_string(),
                    "172.16.254.1".to_string(),
                    "172.162.5.41".to_string(),
                    "172.162.54.1".to_string(),
                ],
            ),
        ];
        for (s, mut want) in testcases {
            want.sort();
            let mut got = Solution::restore_ip_addresses(s);
            got.sort();
            assert_eq!(got, want);
        }
    }
}
