package main

import (
	"testing"
)

func TestMaxSubArray(t *testing.T) {
	inputs := [][]int{
		{-2, 1, -3, 4, -1, 2, 1, -5, 4},
		{1},
		{5, 4, -1, 7, 8},
        {-1},
        {-2,1},
	}
	outputs := []int{
		6,
		1,
		23,
        -1,
        1,
	}
	for i, input := range inputs {
		want := outputs[i]
		got := maxSubArray(input)
		if got != want {
			t.Errorf("For input=%+v\n\tWant=%d but Got=%d",
				input,
				want,
				got,
			)
		}
	}
}
