package main

import (
	"testing"
)

func TestIsPossible(t *testing.T) {

	inputs := [][]int{
		{20, 21, 22},
		{9, 10},
		{1, 2, 3, 3, 4, 4, 5, 5},
		{1, 2, 3, 3, 4, 5},
		{1, 2, 3, 4, 4, 5},
	}
	outputs := []bool{true, false, true, true, false}
	for i, input := range inputs {
		want := outputs[i]
		got := isPossible(input)
		if got != want {
			t.Errorf("For input=%+v Want=%t but Got=%t", input, want, got)
		}
	}
}
