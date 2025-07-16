package main

import "fmt"

func twoSum(nums []int, target int) (result []int) {

	numMap := make(map[int]int, len(nums))
	for i := range nums {
        if v, ok := numMap[target - nums[i]]; ok {
            if i != v {
                return []int{v, i}
            }
        }
        numMap[nums[i]] = i
	}

    return 
}

func main() {
    fmt.Println(twoSum([]int{2,7,11,15}, 9))
    fmt.Println(twoSum([]int{3,2,4},6))
    fmt.Println(twoSum([]int{3,3},6))
}
