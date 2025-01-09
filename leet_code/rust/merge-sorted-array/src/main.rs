struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        if m == 0 {
            for (i, &n) in nums2.iter().enumerate() {
                nums1[i] = n;
            }
            return;
        }
        if n == 0 {
            return;
        }

        let (mut i, mut j, mut k) = (m as usize, n as usize, (m + n - 1) as i32);

        while j > 0 {
            if i > 0 {
                if nums1[i - 1] > nums2[j - 1] {
                    nums1[k as usize] = nums1[i - 1];
                    i -= 1;
                } else {
                    nums1[k as usize] = nums2[j - 1];
                    j -= 1;
                }
            } else {
                nums1[k as usize] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test;

    use super::Solution;

    #[test]
    fn test_merge_solution() {
        let mut test_cases = vec![
            (
                vec![4, 5, 6, 0, 0, 0],
                3,
                vec![1, 2, 3],
                3,
                vec![1, 2, 3, 4, 5, 6],
            ),
            (vec![1], 1, Vec::new(), 0, vec![1]),
            (vec![0], 0, vec![1], 1, vec![1]),
            (
                vec![1, 2, 3, 0, 0, 0],
                3,
                vec![2, 5, 6],
                3,
                vec![1, 2, 2, 3, 5, 6],
            ),
        ];

        for (i, test_case) in test_cases.iter_mut().enumerate() {
            println!("testcase : {i}");
            Solution::merge(&mut test_case.0, test_case.1, &mut test_case.2, test_case.3);
            assert_eq!(test_case.0, test_case.4);
        }
    }
}
