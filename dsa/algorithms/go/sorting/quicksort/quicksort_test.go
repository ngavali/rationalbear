package main

import (
	"fmt"
	"testing"
)

func BenchmarkQuicksort(b *testing.B) {

	for i := 0; i < b.N; i++ {
        nums := []int{5, 3, 6, 1, 9, 10, 4, 7, 8, 2}
		quicksort(nums)
	}

}

func TestQuicksort(t *testing.T) {
	nums := [][]int{
        {5, 3, 6, 1, 9, 10, 4, 7, 8, 2},
        {2,0,2,1,1,0},
    }
    for i := range nums {
	    quicksort(nums[i])
	    fmt.Println(nums[i])
    }

}
