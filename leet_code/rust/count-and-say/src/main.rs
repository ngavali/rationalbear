//https://leetcode.com/problems/count-and-say/
struct Solution;

impl Solution {
    fn count_and_say_recursive(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let s = Self::count_and_say_recursive(n-1);
        let mut s = s.chars();
        let mut ans = String::from("");
        let mut curr = s.next();
        while curr != None {
            let (mut j, mut n) = (0, curr);
            while n == curr {
                n = s.next();
                j+=1;
            }
            ans.push_str(format!("{j}{}",curr.unwrap()).as_str());
            curr = n;
        }
        ans
    }
    pub fn count_and_say(n: i32) -> String {
        Self::count_and_say_recursive(n)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(i32, String)> {
        vec![(4, "1211".to_string()), (1, "1".to_string())]
    }
    #[test]
    fn test_count_and_say() {
        for (n, want) in testcases() {
            assert_eq!(Solution::count_and_say(n), want);
        }
    }
}
