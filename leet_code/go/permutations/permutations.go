package main

import "fmt"

func permutate(nums []int) {
	if len(nums) > 0 {
		//		fmt.Printf("%+v\n", nums)
		return
		for i := 0; i < len(nums); i++ {
			permutate(append(nums[:i], nums[i+1:]...))
		}
	}
}

func permute(nums []int) [][]int {

	res := [][]int{}
	fmt.Println(nums)

	for i := 0; i < len(nums); i++ {
		fmt.Printf("%+v: %d,%d %+v\n", nums, i, i+1, append(nums[:i], nums[i+1:]...))
		permutate(append(nums[:i], nums[i+1:]...))
	}

	return res

}

func main() {
	fmt.Println(permute([]int{1, 2, 3}))
}
