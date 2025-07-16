package main

import (
	"fmt"
	"sort"
)

/*
//Best answer
func merge(intervals [][]int) [][]int {

    sort.Slice(intervals, func(i, j int) bool {
        return intervals[i][0] < intervals[j][0]
    })

    res, start, end := [][]int{}, intervals[0][0], intervals[0][1]

    for i := 1; i < len(intervals); i++ {

        s, e := intervals[i][0], intervals[i][1]

        if s > end {
            res = append(res, []int{start, end})
            start, end = s, e
        } else if (e >= end) {
            end = e
        }
    }

    res = append(res, []int{start, end})

    return res
}
*/

func maxOf(x, y int) int {
    if x > y {
        return x
    }
    return y
}

func minOf(x, y int) int {
    if x < y {
        return x
    }
    return y
}

func merge(intervals [][]int) ([][]int) {

    sort.Slice(intervals, func(i, j int) bool {
        return intervals[i][0] < intervals[j][0]// && intervals[i][1] < intervals[j][1]
    })

    if len(intervals) > 1 {

        i, curr, next := 1, intervals[0], intervals[1]
        for i<len(intervals) {
            next = intervals[i]
            if curr[1] < next[0] {
                i++
            } else {
                next[0] = minOf(curr[0], next[0])
                next[1] = maxOf(curr[1], next[1])
                intervals = append(intervals[:i-1], intervals[i:]...)
            }
            curr = next
        }
    }


    return intervals

}

func main() {
    inputs := [][][]int{
        {{2,3},{4,5},{6,7},{8,9},{1,10}},
        {{2,6},{1,3},{15,18},{8,10}},
        {{1,4},{4,5}},
    }

    for _, intervals := range inputs {
        fmt.Println(merge(intervals))
    }
}
