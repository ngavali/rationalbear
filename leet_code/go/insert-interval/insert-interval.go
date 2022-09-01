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

    start := newInterval[0]
    N := len(intervals) - 1
    var final [][]int
    for i := range intervals {
        if start >= intervals[N-i][0] {
            final = make([][]int, len(intervals[:N-i+1]))
            copy(final, intervals[:N-i+1])
            final = append(final, newInterval)
            final = append(final, intervals[N-i+1:]...)
            break
        }
    }

    i := 0
    for {
        if i > N-1 {
            break
        }
        num := final[i]
        numNext := final[i+1]
        if num[1] > numNext[0] {
            fmt.Println(num, numNext)
            start = i
            final[i][1] = maxOf(num[1], numNext[1])
            final[i+1][1] = maxOf(num[1], numNext[1])
            final = append(final[:i+1], final[i+2:]...)
        }
        i++
    fmt.Println(final)
    }

    return nil
}

func main() {
    //fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{2, 5}))
    //fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{4, 5}))
    fmt.Println(insert([][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, []int{4, 8}))
}
