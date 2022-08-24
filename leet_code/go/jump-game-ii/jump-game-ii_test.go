package main

import "testing"

func TestJump(t *testing.T) {
	input_collection := [][]int{
		{0},
		{3, 2, 1},
		{2, 3, 1, 1, 4},
	}
	output_collection := []int{
		0,
		1,
		2,
	}

	for i, input := range input_collection {
		want := output_collection[i]
		got := jump(input)
		if want != got {
			t.Errorf("Input: %+v, want %d, but got %d", input, want, got)
		}
	}

}
