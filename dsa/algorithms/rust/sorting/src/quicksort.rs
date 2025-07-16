///Quick sort function on vector of i32
pub fn do_sort(nums: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let (mut pivot, mut i) = (low, low);

        while i < high {
            if nums[i] < nums[high] {
                if i != pivot {
                    (nums[pivot], nums[i]) = (nums[i], nums[pivot]);
                }
                pivot += 1;
            }
            i += 1;
        }

        (nums[pivot], nums[high]) = (nums[high], nums[pivot]);

        if pivot > 0 {
            do_sort(nums, low, pivot - 1);
        }
        do_sort(nums, pivot + 1, high);
    }
}

pub fn sort(nums: &mut [i32]) {
    if !nums.is_empty() {
        let (mut pivot, high, mut i) = (0, nums.len() - 1, 0);
        while i < high {
            if nums[i] < nums[high] {
                if i != pivot {
                    (nums[pivot], nums[i]) = (nums[i], nums[pivot]);
                }
                pivot += 1;
            }
            i += 1;
        }

        (nums[pivot], nums[high]) = (nums[high], nums[pivot]);

        sort(&mut nums[..pivot]);
        sort(&mut nums[pivot + 1..]);
    }
}

#[test]
fn test_do_sort() {
    let mut nums = vec![9, 10, 2, 8, 3, 4, 6, 7, 5, 1, 3];
    let n = nums.len();
    do_sort(&mut nums, 0, n - 1);
    assert_eq!(nums, vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_sort() {
    let mut nums = vec![9, 10, 2, 8, 3, 4, 6, 7, 5, 1, 3];
    sort(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10]);
}
