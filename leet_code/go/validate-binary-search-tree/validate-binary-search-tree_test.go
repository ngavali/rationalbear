package main

import (
	"testing"
)

func TestIsValidBST(t *testing.T) {

	inputs := []*TreeNode{
		&TreeNode{0, nil, nil},
		&TreeNode{2, &TreeNode{2, nil, nil}, &TreeNode{2, nil, nil}},
		&TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}},
		&TreeNode{5, &TreeNode{1, nil, nil}, &TreeNode{4, &TreeNode{3, nil, nil}, &TreeNode{6, nil, nil}}},
		&TreeNode{5, &TreeNode{4, nil, nil}, &TreeNode{6, &TreeNode{3, nil, nil}, &TreeNode{7, nil, nil}}},
	}
	outputs := []bool{
		true,
		false,
		true,
		false,
		false,
	}
	for i, input := range inputs {
		got := isValidBST(input)
		want := outputs[i]
		if got != want {
			t.Errorf("For input[%d]: %+v, want: %t, got: %t", i, input, want, got)
		}
	}
}
