package main

import "fmt"

func threeSum(nums []int, target int) (ans [][]int) {

	num_map := make(map[int]int)
	for k, v := range nums {
		num_map[v] = k
	}
	fmt.Println(num_map)

	for i := 0; i < len(nums)-2; i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			if k, ok := num_map[0-(nums[i]+nums[j])]; ok {
				if i != j && i != k && j != k {
					ans = append(ans, []int{nums[i], nums[j], nums[k]})
				}
			}
		}
	}

	return
}

func main() {
	fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4}, 1))
	fmt.Println(threeSum([]int{0, 0, 0}, 2))
	fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4}, 3))
	/*[[-4,0,4],[-4,1,3],[-3,-1,4],[-3,0,3],[-3,1,2],[-2,-1,3],[-2,0,2],[-1,-1,2],[-1,0,1]]*/
}
