//https://leetcode.com/problems/count-and-say/
struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ans = String::from("1");
        if n != 1 {
            for _ in 1..n {
                let mut c_ans = String::with_capacity(ans.len() * 2);
                let mut s = ans.chars();
                let mut curr = s.next();
                let (mut j, mut n) = (0, curr);
                while curr != None {
                    if n == curr {
                        n = s.next();
                        j += 1;
                    } else {
                        //Handle last value too due to None check
                        c_ans.push_str(format!("{j}{}", curr.unwrap()).as_str());
                        curr = n;
                        j = 0;
                    }
                }
                ans = c_ans;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testcases() -> Vec<(i32, String)> {
        vec![
            (4, "1211".to_string()),
            (1, "1".to_string()),
            (12, "3113112221232112111312211312113211".to_string()),
            (
                15,
                "311311222113111231131112132112311321322112111312211312111322212311322113212221"
                    .to_string(),
            ),
        ]
    }
    #[test]
    fn test_count_and_say() {
        for (n, want) in testcases() {
            assert_eq!(Solution::count_and_say(n), want);
        }
    }
}
