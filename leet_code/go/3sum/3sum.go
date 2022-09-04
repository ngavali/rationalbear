package main

import (
	"fmt"
	"sort"
)

func threeSum(nums []int) (ans [][]int) {
    sort.Slice(nums, func(i, j int) bool {
        return nums[i] < nums[j]
    })
    for i := 0; i < len(nums); i++ {
        if i != 0 && nums[i] == nums[i-1] {
            continue
        }
        j, k := i+1, len(nums)-1
        for j < k {
            sum := nums[i]+nums[j]+nums[k]
            if sum == 0 {
                ans = append(ans, []int{nums[i], nums[j], nums[k]})
                j++
                for j < k && nums[j] == nums[j-1] {
                    j++
                }
                k--
            } else if sum < 0 {
                j++
            } else if sum > 0 {
                k--
            }
        }
    }
    return
}

var usedNumI = map[int]bool{}

func twoSum(nums []int, target, idx int) (result [][]int) {

    usedNumJ := make(map[int]bool, len(nums))
    numMap := make(map[int]int, len(nums))
    for i := idx + 1; i < len(nums); i++ {
        if v, ok := numMap[target-nums[i]]; ok {
            if !usedNumJ[nums[i]] && !usedNumI[nums[v]] && !usedNumI[nums[i]] {
                usedNumJ[nums[i]] = true
                usedNumJ[nums[v]] = true
                result = append(result, []int{nums[v], nums[i]})
            }
        }
        numMap[nums[i]] = i
    }

    return
}

func XthreeSum(nums []int) (ans [][]int) {

    /*
    sort.Slice(nums, func(i, j int) bool {
        return nums[i] < nums[j]
    })
    res := [][]int{}
    prev := -10000
    */
    usedNumI = make(map[int]bool, len(nums))
    for i := range nums {
        /*if prev == nums[i] {
            continue
        }*/
        if !usedNumI[nums[i]] {
            sum := [][]int{}
            for _, r := range twoSum(nums, -nums[i], i) {
                if len(r) == 2 {
                    sum = append(sum, append([]int{nums[i]}, r...))
                }
            }
            ans = append(ans, sum...)
        }
        usedNumI[nums[i]] = true
        //prev = nums[i]
    }

    return
}

func YthreeSum(nums []int) (ans [][]int) {

    sort.Slice(nums, func(i, j int) bool {
        return nums[i] < nums[j]
    })
    for i := 0; i < len(nums); i++ {
        if i != 0 && nums[i] == nums[i-1] {
            continue
        }
        for j := i + 1; j < len(nums); j++ {
            if j != i+1 && nums[j] == nums[j-1] {
                continue
            }
            for k := j + 1; k < len(nums); k++ {
                if k != j+1 && nums[k] == nums[k-1] {
                    continue
                }
                if nums[i]+nums[j]+nums[k] == 0 {
                    ans = append(ans, []int{nums[i], nums[j], nums[k]})
                }
            }
        }
    }
    return 
}

func main() {
    fmt.Println(threeSum([]int{-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0}))
    fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4}))
    fmt.Println(threeSum([]int{0, 0, 0, 0}))
    fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4}))
    fmt.Println(threeSum([]int{0, 0, 0}))
    fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4}))
    /*[[-4,0,4],[-4,1,3],[-3,-1,4],[-3,0,3],[-3,1,2],[-2,-1,3],[-2,0,2],[-1,-1,2],[-1,0,1]]*/
}
