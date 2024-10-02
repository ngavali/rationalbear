struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (N1, N2, mut i, mut j, mut k) = (nums1.len(), nums2.len(), 0, 0, 0);
        let total_len = N1 + N2;
        let mut N = total_len / 2 + 1;
        let mut sort_nums: Vec<i32> = Vec::with_capacity(usize::from(N));

        while i < N1 && j < N2 && k < N {
            if nums1[i] < nums2[j] {
                sort_nums.push(nums1[i]);
                i += 1;
            } else {
                sort_nums.push(nums2[j]);
                j += 1;
            }
            k += 1;
        }
        while k < N {
            if i < N1 {
                sort_nums.push(nums1[i]);
                i += 1;
            }
            if j < N2 {
                sort_nums.push(nums2[j]);
                j += 1;
            }
            k += 1;
        }

        return if total_len % 2 == 0 {
            (sort_nums[N - 2] as f64 + sort_nums[N - 1] as f64) / (2 as f64)
        } else {
            sort_nums[N - 1] as f64
        };
    }
}

fn main() {
    println!(
        "{} ",
        Solution::find_median_sorted_arrays(vec![], vec![2, 3])
    );
    println!(
        "{} ",
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
    println!(
        "{} ",
        Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![3, 4])
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_medians() {
        let test_cases = vec![
            (vec![vec![3], vec![-2, -1]], -1.0),
            (vec![vec![], vec![2, 3]], 2.5),
            (vec![vec![2, 3], vec![]], 2.5),
            (vec![vec![1, 2], vec![3, 4]], 2.5),
            (vec![vec![1, 2, 3], vec![3, 4]], 3f64),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::find_median_sorted_arrays(test_case.0[0].clone(), test_case.0[1].clone()),
                test_case.1
            );
        }
    }
}
