package main

import "testing"

func TestKthSmallest(t *testing.T) {

	input := &TreeNode{
		5,
		&TreeNode{3,
			&TreeNode{2,
				&TreeNode{1, nil, nil},
				nil,
			},
			&TreeNode{4, nil, nil},
		},
		&TreeNode{6, nil, nil},
	}

    output := 3

    got := kthSmallest(input, 3)
    if output != got {
	    t.Errorf("For input, want=%d, but got=%d\n", output, got)
    }
}
