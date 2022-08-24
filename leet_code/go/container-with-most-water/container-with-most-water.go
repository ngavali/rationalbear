package main

import "fmt"

func MIN(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func MAX(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Area(h1, h2, w int) int {
	return MIN(h1, h2) * w
}

func maxArea(height []int) (area int) {
	i := 0
	j := len(height) - 1

	for i <= j {
		fmt.Println(height[i], height[j])
		current_area := Area(height[i], height[j], j-i)
		area = MAX(current_area, area)
		if height[i] < height[j] {
			i++
		} else {
			j--
		}
	}
	return
}

func main() {
	fmt.Println(maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7}))
	//fmt.Println(maxArea([]int{1, 1}))
}
