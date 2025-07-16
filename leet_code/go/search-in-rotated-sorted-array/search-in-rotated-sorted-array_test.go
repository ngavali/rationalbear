package main

import (
	"testing"
)

func TestSearch(t *testing.T) {
    inputs := [][]int{
        {1, 1},
        {0, 6, 7, 0, 1, 2, 4, 5},
        {0, 4, 5, 6, 7, 0, 1, 2},
        {3, 4, 5, 6, 7, 0, 1, 2},
        {0, 1},
    }
    outputs := []int{
        0,
        2,
        4,
        -1,
        -1,
    }
    for i, input := range inputs {
        want := outputs[i]
        got := search(input[1:], input[0])
        if got != want {
            t.Errorf("For input=%+v target=%d\n\twant=%d but got=%d", input[1:], input[0], want, got)
        }
    }
}
