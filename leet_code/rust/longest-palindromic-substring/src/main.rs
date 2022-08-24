struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut r : u8 = s.as_bytes()[0];
        let mut s = s.as_bytes();
        let s_len = s.len();
        let mut i = 1;
        while i < s_len {
            println!("{}", r);
            r = r ^ s[i];
            i+=1;
        }
        
            println!("{}", r);
        "abc".to_string()
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("bab".to_string()));
}
