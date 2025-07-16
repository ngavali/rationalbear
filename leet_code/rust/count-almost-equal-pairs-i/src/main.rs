struct Solution;

impl Solution {
    fn append_zeros(num: Vec<u8>, repeat: usize) -> Vec<u8> {
        if repeat > 0 {
            let mut zero_prefixed_num = std::iter::repeat("0")
                .take(repeat)
                .collect::<String>()
                .as_bytes()
                .to_vec();

            for c in num.into_iter() {
                zero_prefixed_num.push(c);
            }

            return zero_prefixed_num;
        }
        num
    }

    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut matches = 0;
        for (i, &num_1) in nums.iter().enumerate() {
            for &num_2 in nums[i + 1..].iter() {
                if Solution::check_pair(num_1, num_2) || num_1 == num_2 {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn check_pair(num_1: i32, num_2: i32) -> bool {
        let (mut num_1, mut num_2) = (
            num_1.to_string().as_bytes().to_vec(),
            num_2.to_string().as_bytes().to_vec(),
        );
        let (num_1_len, num_2_len) = (num_1.len(), num_2.len());

        if num_1_len < num_2_len {
            num_1 = Solution::append_zeros(num_1, num_2_len - num_1_len);
        } else if num_2_len < num_1_len {
            num_2 = Solution::append_zeros(num_2, num_1_len - num_2_len);
        }

        let mut diff_index = Vec::with_capacity(3);
        for (i, _) in num_1.iter().enumerate() {
            if num_1[i] != num_2[i] {
                diff_index.push(i);
            }
            if diff_index.len() > 2 {
                return false;
            }
        }

        match diff_index.len() {
            2 => {
                num_1[diff_index[0]] == num_2[diff_index[1]]
                    && num_1[diff_index[1]] == num_2[diff_index[0]]
            }
            _ => return false,
        }

        /*
        if diff_index.len() == 2 {
            (num_1[diff_index[0]], num_1[diff_index[1]]) =
                (num_1[diff_index[1]], num_1[diff_index[0]]);
            if num_1 == num_2 {
                return true;
            }
        }

        false
        */
    }
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_count_pairs() {
        let test_cases = vec![
            (vec![3, 12, 30, 17, 21], 2),
            (vec![1, 1, 1, 1, 1], 10),
            (vec![123, 231], 0),
        ];

        for test_case in test_cases {
            assert_eq!(test_case.1, Solution::count_pairs(test_case.0));
        }
    }
}
