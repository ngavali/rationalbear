package main

import (
	"testing"
)

func TestDiameterOfBinaryTree(t *testing.T) {
	inputs := []*TreeNode{
		{1,
			&TreeNode{2,
				&TreeNode{4, nil, nil},
				&TreeNode{5, nil, nil},
			},
			&TreeNode{3, nil, nil},
		},
        {1,
            &TreeNode{2,nil,nil},
            nil,
        },
	}
    outputs := []int{
        3,
        1,
    }

    for i, input := range inputs {
        want:=outputs[i]
        got:= diameterOfBinaryTree(input)
        if got!=want {
            t.Errorf("For input[%d], Want=%d but got=%d", i, want, got)
        }
    }
}
