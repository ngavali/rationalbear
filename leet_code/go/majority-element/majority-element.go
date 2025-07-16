package main

import "fmt"

func majorityElement(nums []int) int {
	vote := 0      
	candidate := 0 

	for _, n := range nums {
		if vote == 0 {
			candidate = n
		}
		if n == candidate {
			vote++
		} else {
			vote--
		}

	}
	return candidate
}

func main() {
	fmt.Println(majorityElement([]int{2, 2, 2, 2, 1, 1, 1}))

}
