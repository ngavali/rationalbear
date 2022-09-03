package main

import (
	"fmt"
	"sort"
)


type location struct {
    pos int
    idx int
}

func kClosest(points [][]int, k int) [][]int {

    l := []location{}

    for i := range points {
        l = append(l, location{points[i][0]*points[i][0] + points[i][1]*points[i][1], i})
    }

    sort.Slice(l, func(i, j int) bool {
        return l[i].pos < l[j].pos
    })
    res := [][]int{}
    i := 0

    for i < k {
        res = append(res, points[l[i].idx])
        i++
    }
    return res
}

func main() {
    fmt.Println(kClosest([][]int{{1, 3}, {-2, 2}}, 1))
    fmt.Println(kClosest([][]int{{3, 3}, {5, -1}, {-2, 4}}, 2))
}
