package main

import "testing"

func TestRightSideView(t *testing.T) {

	inputs := []*TreeNode{
		{1,
			&TreeNode{2,
				&TreeNode{0, nil, nil},
				&TreeNode{5, nil, nil},
			},
			&TreeNode{3,
				nil,
				&TreeNode{4, nil, nil},
			},
		},
		{1,
			&TreeNode{2,
				&TreeNode{4, nil, nil},
				nil},
			&TreeNode{3, nil, nil},
		},
	}

	outputs := [][]int{
		{1, 3, 4},
		{1, 3, 4},
	}

	for i, input := range inputs {
		want := outputs[i]
		got := rightSideView(input)
		for j := range got {
			if want[j] != got[j] {
				t.Errorf("For input[%d], want=%+v but got=%+v\n", i, want, got)
			}
		}
	}

}
