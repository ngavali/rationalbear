package main

import (
	"fmt"
)

func doPartition(nums []location, low, high int ) int {

    for i:=low; i<high; i++ {
        if nums[i].pos < nums[high].pos {
            if i != low {
                fmt.Println("Swapped:", nums[i], nums[low])
                nums[i], nums[low] = nums[low], nums[i]
            }
            low++
        }
    }

    nums[low], nums[high] = nums[high], nums[low]

    return low

}

func quicksort(nums []location, low, high, pos int) []int {

    if low < high {
        partition := doPartition(nums, low, high)
        quicksort(nums, low, partition-1, pos)
        if partition < pos-1 {
            quicksort(nums, partition+1, high, pos)
        }
    }
    return nil
}


type location struct {
    pos int
    idx int
}

func kClosest(points [][]int, k int) [][]int {

    l := make([]location, len(points))

    for i := range points {
        l = append(l, location{points[i][0]*points[i][0] + points[i][1]*points[i][1], i})
    }

    fmt.Printf("Before: %+v\n",l)
    quicksort(l, 0, len(l)-1,k)
    fmt.Printf("Sorted: %+v\n",l)
    /*
    sort.Slice(l, func(i, j int) bool {
        return l[i].pos < l[j].pos
    })*/

    res := [][]int{}
    i := 0

    for i < k {
        res = append(res, points[l[i].idx])
        i++
    }
    return res
}

/*best solution
func kClosest(points [][]int, k int) [][]int {
    sqrDistance := func(coord []int) int {
        return coord[0]*coord[0] + coord[1]*coord[1]
    }

    partition := func(l, r int) int {
        index := l + rand.Intn(r-l)
        pivotPoint := points[index]
        pivotDistance := sqrDistance(pivotPoint)

        points[index], points[r] = points[r], points[index]

        storeIndex := l
        for i := l; i <= r; i++ {
            if d := sqrDistance(points[i]); d > pivotDistance {
                points[storeIndex], points[i] = points[i], points[storeIndex]
                storeIndex++
            }
        }

        points[storeIndex], points[r] = points[r], points[storeIndex]

        return storeIndex
    }

    n := len(points)
    kSmallest := n - k
    for l, r := 0, n-1; l < r; {
        pivotIndex := partition(l, r)

        switch {
        case kSmallest == pivotIndex:
            return points[kSmallest:n]
        case kSmallest < pivotIndex:
            r = pivotIndex - 1
        case kSmallest > pivotIndex:
            l = pivotIndex + 1
        }
    }
    return points[kSmallest:n]
}
*/

func main() {
    fmt.Println(kClosest([][]int{{1, 3}, {-2, 2}}, 1))
    fmt.Println(kClosest([][]int{{3, 3}, {5, -1}, {-2, 4}}, 2))

}
