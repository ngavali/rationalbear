package main

import "fmt"

func simpleSort(a []int) {
	for i := 0; i < len(a); i++ {
		for j := i; j < len(a); j++ {
			if a[i] < a[j] {
				a[i], a[j] = a[j], a[i]
			}
		}
	}
}

func findMax(arr []int, kth int) int {
	max_num := 0
	num_map := make(map[int]int)
	for i := 0; i < len(arr); i++ {
		if max_num < arr[i] {
			max_num = arr[i]
		}
		if val, ok := num_map[arr[i]]; ok {
			num_map[arr[i]] = val + 1
		} else {
			num_map[arr[i]] = 1
		}
	}
	for {
		if val, ok := num_map[max_num]; ok {
			kth -= val
		}
		max_num -= 1
		if kth == 1 {
			return max_num
		}
	}
	//fmt.Printf("MAX VAL:%+v\nMAX QUEUE:%+v\n", max_num, num_map)
	return kth
}

func main() {
	var arr = []int{3, 2, 1, 5, 6, 4}
	fmt.Printf("%+v\n", findMax(arr, 2))
	arr = []int{3, 2, 3, 1, 2, 4, 5, 5, 6}
	fmt.Printf("%+v\n", findMax(arr, 4))
}
