package main

import (
	"testing"
)

func TestLowestCommonAncestor(t *testing.T) {
	inputs := []*TreeNode{
		{6,
			&TreeNode{2,
				&TreeNode{0, nil, nil},
				&TreeNode{4,
					&TreeNode{3, nil, nil},
					&TreeNode{5, nil, nil},
				}},
			&TreeNode{8,
				&TreeNode{7, nil, nil},
				&TreeNode{9, nil, nil},
			},
		},
        {3,
            &TreeNode{5,
                &TreeNode{6, nil, nil},
                &TreeNode{2,
                    &TreeNode{7, nil, nil},
                    &TreeNode{4, nil, nil},
                },
            },
            &TreeNode{1,
                &TreeNode{0, nil, nil},
                &TreeNode{8, nil, nil},
            },
        },
        {3,
            &TreeNode{5,
                &TreeNode{6, nil, nil},
                &TreeNode{2,
                    &TreeNode{7, nil, nil},
                    &TreeNode{4, nil, nil},
                },
            },
            &TreeNode{1,
                &TreeNode{0, nil, nil},
                &TreeNode{8, nil, nil},
            },
        },
	}
	params := [][]*TreeNode{
		{&TreeNode{2, nil, nil}, &TreeNode{4, nil, nil}},
		{&TreeNode{5, nil, nil}, &TreeNode{1, nil, nil}},
		{&TreeNode{5, nil, nil}, &TreeNode{4, nil, nil}},
	}
	outputs := []*TreeNode{
		{2, nil, nil},
		{3, nil, nil},
		{5, nil, nil},
	}
	for i, input := range inputs {
		want := outputs[i]
		got := lowestCommonAncestor(input, params[i][0], params[i][1])
		if got.Val != want.Val {
			t.Errorf("want %+v but got %v\n", want, got)
		}
	}
}
