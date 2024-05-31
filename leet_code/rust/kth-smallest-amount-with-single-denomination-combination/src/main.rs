fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        (a, b) = (b, a);
    }

    let mut t = 0;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let mut number = 0;
    number
}

/*
pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    //pub fn find_kth_smallest_slow(coins: Vec<i32>, k: i32) -> i64 {
    let mut i: i64 = 1;
    let mut index = 1;
    let mut number: i64 = 0;
    while index <= k {
        for coin in coins.iter() {
            if i as i32 % coin == 0 {
                index += 1;
                number = i;
                break;
            }
        }
        i += 1;
    }
    number
}
*/

fn main() {}

#[cfg(test)]
mod test {
    use crate::{find_kth_smallest, gcd, lcm};

    #[test]
    fn check_find_kth_smallest() {
        let test_cases = vec![
            (vec![3, 6, 9], 3, 9),
            (vec![5, 2], 7, 12),
            (vec![6, 1, 2, 4], 4, 4),
            //           (vec![6, 5], 1435065516, 4305196551),
        ];
        for (input_0, input_1, exp_output) in test_cases {
            assert_eq!(find_kth_smallest(input_0, input_1), exp_output);
        }
    }

    #[test]
    fn check_gcd() {
        let test_cases = vec![
            (vec![2, 3], 1),
            (vec![3, 6], 3),
            (vec![10, 12], 2),
            (vec![15, 9], 3),
        ];
        for (input, exp_output) in test_cases {
            assert_eq!(gcd(input[0], input[1]), exp_output);
        }
    }
    #[test]
    fn check_lcd() {
        let test_cases = vec![
            (vec![3, 15], 15),
            (vec![15, 20], 60),
            (vec![5, 7], 35),
            (vec![24, 36], 72),
        ];
        for (input, exp_output) in test_cases {
            assert_eq!(lcm(input[0], input[1]), exp_output);
        }
    }
}
