package main

import "fmt"

func minOf(x, y int) int {
	if x > y {
		return y
	}
	return x
}

func maxOf(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func insert(intervals [][]int, newInterval []int) [][]int {

	if len(intervals) == 0 {
		return append(intervals, newInterval)
	}
	start := newInterval[0]
	//N := len(intervals) - 1

	insert_location := 0

	for i := range intervals {
		if start > intervals[i][0] {
			insert_location = i + 1
			fmt.Println(start, intervals[i][0], insert_location)
		}
	}

	final := make([][]int, len(intervals[:insert_location]))
	copy(final, intervals[:insert_location])
	final = append(final, newInterval)
	final = append(final, intervals[insert_location:]...)
	fmt.Println(final, intervals)
	/*
		for i := range intervals {
			if start > intervals[N-i][0] {
				fmt.Println(final, intervals)
				final = make([][]int, len(intervals[:N-i+1]))
				copy(final, intervals[:N-i+1])
				final = append(final, newInterval)
				final = append(final, intervals[N-i+1:]...)
				break
			}
		}*/

	i := 0
	for i < len(final)-1 {
		num := final[i]
		numNext := final[i+1]
		if num[1] >= numNext[0] {
			fmt.Println(num, numNext)
			start = i
			final[i][1] = maxOf(num[1], numNext[1])
			final[i+1][1] = maxOf(num[1], numNext[1])
			final = append(final[:i+1], final[i+2:]...)
		} else {
			i++
			fmt.Println(final)
		}
	}

	return final
}

func main() {
	fmt.Println("FINAL=", insert([][]int{{1, 5}}, []int{0, 3}))
	fmt.Println("FINAL=", insert([][]int{}, []int{5, 7}))
	fmt.Println("FINAL=", insert([][]int{{1, 3}, {6, 9}}, []int{2, 5}))
	fmt.Println("FINAL=", insert([][]int{{1, 3}, {6, 9}}, []int{4, 5}))
	fmt.Println("FINAL=", insert([][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, []int{4, 8}))
	fmt.Println("FINAL=", insert([][]int{{1, 3}, {6, 9}}, []int{11, 12}))
}
