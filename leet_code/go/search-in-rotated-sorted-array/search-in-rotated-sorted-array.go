package main

/*
implement O(log N) solution
*/

func binarySearchLogNLoop(nums []int, target, low, high int) (mid int) {

    for low <= high {
        mid = (high + low) / 2

        if target == nums[mid] {
            return mid
        }

        if nums[low] <= nums[mid] { //Left side has a sorted array check in that range
            if nums[low] <= target && target < nums[mid] {
                //equality is already checked above
                high = mid - 1
            } else {
                low = mid + 1
            }
        } else { // Right side has a sorted array check in that range
            if nums[mid] < target && target <= nums[high] {
                //equality is already checked above
                low = mid + 1
            } else {
                high = mid - 1
            }

        }

    }

    return -1

}

func binarySearchLogN(nums []int, target, low, high int) (mid int) {
    if low <= high {

        mid = (high + low) / 2
        if target == nums[mid] {
            return mid
        }
        if nums[low] <= nums[mid] { //Left side has a sorted array check in that range
            if nums[low] <= target && target <= nums[mid] {
                return binarySearchLogN(nums, target, low, mid-1)
            } else {
                return binarySearchLogN(nums, target, mid+1, high)
            }
        } else { // Right side has a sorted array check in that range
            if nums[mid] <= target && target <= nums[high] {
                return binarySearchLogN(nums, target, mid+1, high)
            } else {
                return binarySearchLogN(nums, target, low, mid-1)
            }

        }
    }
    return -1
}

func binarySearchX(nums []int, target, low, high int) int {

    if low < high {
        mid := (high + low) / 2

        if target == nums[mid] {
            return mid
        }
        if target < nums[mid] {
            return binarySearchX(nums, target, low, mid)
        } else {
            return binarySearchX(nums, target, mid+1, high)
        }
    }
    return -1

}

func searchN(nums []int, target int) int {

    pivot := 0
    for i := range nums {
        if nums[pivot] < nums[i] {
            pivot = i
        }
    }
    if target == nums[pivot] {
        return pivot
    }
    if target == nums[0] {
        return 0
    }
    if nums[0] < target && target < nums[pivot] {
        return binarySearchX(nums, target, 0, pivot)
    } else {
        return binarySearchX(nums, target, pivot+1, len(nums))
    }

}

func search(nums []int, target int) int {
    //return binarySearchLogN(nums, target, 0, len(nums)-1)
    return binarySearchLogNLoop(nums, target, 0, len(nums)-1)
}
