struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let mut N = total_len/2+1;
        //total_len = if total_len == 2 { 4 } else { total_len };
        //println!("{}", total_len);
        let mut sort_nums: Vec<i32> = Vec::with_capacity(usize::from(N));
        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < nums1.len() && j < nums2.len() && k < N {
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
            if i < nums1.len() {
                sort_nums.push(nums1[i]);
                i += 1;
            }
            if j < nums2.len() {
                sort_nums.push(nums2[j]);
                j += 1;
            }
            k += 1;
        }

        return if total_len % 2 == 0 { (sort_nums[N - 2] as f64 + sort_nums[N-1] as f64) / (2 as f64) } else { sort_nums[N-1] as f64 };
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
