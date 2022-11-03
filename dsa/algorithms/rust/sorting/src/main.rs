mod quicksort;

fn main() {
    let mut nums = vec![9, 10, 2, 8, 4, 6, 3, 7, 5, 1, 3];
    let n = nums.len();
    //quicksort::do_sort(&mut nums, 0, n - 1);
    quicksort::sort(&mut nums);
    println!("Sorted nums: {:?}", nums);
}
