package main

import "fmt"

func permute(nums []int) [][]int {

    if len(nums) == 1 {
        return [][]int{nums}
    }

    res := [][]int{}
    if len(nums) == 2 {
        
        res = append(append(res, nums), []int{nums[1], nums[0]})

    } else {

        for i := 0; i < len(nums); i++ {

            subset := make([]int, len(nums))
            copy(subset, nums)
            key := subset[i]
            for _, val := range permute(append(subset[:i], subset[i+1:]...)) {
                res = append(res, append(val, key))
            }

        }

    }

    return res
}

func main() {

    nums := []int{1, 2, 3}
    //nums := []int{1, 2, 3, 4}
    //nums := []int{0}
    //nums := []int{0,1}
    fmt.Println(permute(nums))
}
