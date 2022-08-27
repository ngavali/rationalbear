package main

func maxSubArray(nums []int) int {
    maxSum, currSum := -10000, 0
    for _,v := range nums {
        if currSum < 0 {
            currSum = 0
        }
        currSum += v
        if maxSum < currSum {
            maxSum = currSum
        }
    }
    return maxSum
}
