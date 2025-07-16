struct Solution {}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut y = x;
        let mut z = 0;
        while y!=0 {
            z = z*10 + y%10;
            y/=10;
        }
        x == z
    }
    /*
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut nums : Vec<i32> = Vec::new();
        while x != 0 {
            nums.push(x%10);
            x /=10
        }
        let mut i = 0;
        let nums_len = nums.len();
        while i < nums_len/2 {
            if nums[i] != nums[nums_len-i-1] {
             return false;
            }
            i+=1;
        }
        true
    }
    */
}

fn main() {
    assert_eq!(true, Solution::is_palindrome(121));
    assert_eq!(false, Solution::is_palindrome(-121));
    assert_eq!(false, Solution::is_palindrome(10));
}
