package main

import (
	"testing"
)

func TestRemoveElement(t *testing.T) {
	inputs := [][]int{
		{2, 3, 2, 2, 3},
		{2, 0, 1, 2, 2, 3, 0, 4, 2},
	}
	output := []int{2, 5}
	for i, input := range inputs {
		want := output[i]
		got := removeElement(input[1:], input[0])
		if got != want {
			t.Errorf("For input=%+v, want=%d, but got=%d", input[1:], want, got)
		}
	}

}
