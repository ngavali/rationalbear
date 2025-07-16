struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut num : i32 = 0;
        let observed : u8;
        let ( mut signed, mut sign_observed, mut overflow ) : (bool, bool, bool) = ( false, false, false );
        let mut overflow_checker : i32 = 0;
        for &c in s.as_bytes() {
            match c as char {
                '0'..='9' => {
                    let diff = c as i32 - 48;
                    overflow_checker = num*10 + diff;
                    //println!("{:12} {:12}", num, overflow_checker/10);
                    if (num*10)/10 != num || num != (num*10 + diff)/10 {
                        println!("Overflow detected {}", signed );
                        overflow = true;
                        break;
                    }
                    num = num*10 + diff;
                    sign_observed = true;
                },
                '-' => {
                    signed = true;
                    if sign_observed {
                        break;
                    }
                    sign_observed = true;
                },
                '+' => {
                    if sign_observed {
                        break;
                    }
                    sign_observed = true;
                    continue;
                },
                ' ' => {
                    if sign_observed {
                    break;
                    }
                },
                '.' => {
                    break;
                },
                _ => {
                    break
                },
            };
        }
        if overflow {
            if signed {
                return i32::MIN ;
            }
            return i32::MAX ;
        }
        if signed {
            return -num;
        }
       num
    }
}

fn main() {
    println!("{}", Solution::my_atoi("-42+43".to_string()));
    //let i : i32 = -6147483648;
    //println!("{}", Solution::my_atoi("-6147483648".to_string()));
    //println!("{}", Solution::my_atoi("2147483648".to_string()));
    /*
    println!("{}", Solution::my_atoi("2147483648".to_string()));
    println!("{}", Solution::my_atoi("-042".to_string()));
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
    println!("{}", Solution::my_atoi("+1".to_string()));
    println!("{}", Solution::my_atoi("+-12".to_string()));
    println!("{}", Solution::my_atoi("-+12".to_string()));
    */
}
