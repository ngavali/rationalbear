struct Solution {}

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let (mut signed, mut res) = (1, 0);
        if x < 0 {
            signed = -1;
        }
        while x > 0 {
            if (res*10)/10 != res {
                return 0
            }
            res = res*10 + x%10;
            x = x/10;
        }
        signed * res
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(1534236469));
}
