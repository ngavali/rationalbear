package main

import (
	"testing"
)

func TestSortedArrayToBST(t *testing.T) {
	input := []int{-10, -3, 0, 5, 9}
	/*
			want := `-10
		-3
		0
		5
		9
		`*/
	wanted := []int{}
	//got := &bytes.Buffer{}
	DFS_inorder(sortedArrayToBST(input), &wanted)
	for i := range input {
		if input[i] != wanted[i] {
			t.Errorf("Input: %+v, want='%+v', got='%+v'\n", input, input, wanted)
		}
	}
}
