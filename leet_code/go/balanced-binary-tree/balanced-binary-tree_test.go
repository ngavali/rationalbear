package main

import (
	"testing"
)

func TestIsBalanced(t *testing.T) {

    inputs := []*TreeNode{
        {1,
        &TreeNode{2, &TreeNode{3, &TreeNode{4, nil, nil}, &TreeNode{4, nil, nil}}, &TreeNode{3, nil, nil}},
        &TreeNode{2, nil, nil},
    },
        {3,
        &TreeNode{9, nil, nil},
        &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}},
    },
}

    outputs := []bool{
        false,
        true,
    }

    for i, input := range inputs {
        want := outputs[i]
        got := isBalanced(input)
        if got!=want {
            t.Errorf("For input=%+v\n\tWant=%t but got=%t", input, want, got)
        }
    }


}
