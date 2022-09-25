package main

func quicksort(nums []int) {

    if len(nums) > 1 {
        newPivotIndx, pivot := 0, nums[len(nums)-1]

        for j := range nums {
            if nums[j] < pivot {
                if j != newPivotIndx {
                    nums[j], nums[newPivotIndx] = nums[newPivotIndx], nums[j]
                }
                newPivotIndx++
            }
        }

        nums[newPivotIndx], nums[len(nums)-1] = nums[len(nums)-1], nums[newPivotIndx]

        quicksort(nums[:newPivotIndx])
        quicksort(nums[newPivotIndx+1:])
    }

}
