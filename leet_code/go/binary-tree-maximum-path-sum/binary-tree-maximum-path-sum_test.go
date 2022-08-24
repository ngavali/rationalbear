package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMaxPathSum(t *testing.T) {
	type test_data struct {
		input  *TreeNode
		output int
	}

	cases := make([]test_data, 0)
	//[1,-2,-3,1,3,-2,null,-1]
	//[5,4,8,11,null,13,4,7,2,null,null,null,1]
	cases = append(cases, test_data{&TreeNode{1, &TreeNode{-1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}, &TreeNode{1, nil, nil}}, 4})
	cases = append(cases, test_data{&TreeNode{5, &TreeNode{4, &TreeNode{11, &TreeNode{7, nil, nil}, &TreeNode{2, nil, nil}}, nil}, &TreeNode{8, &TreeNode{13, nil, nil}, &TreeNode{4, nil, &TreeNode{1, nil, nil}}}}, 48})
	cases = append(cases, test_data{&TreeNode{1, &TreeNode{-2, &TreeNode{1, &TreeNode{-1, nil, nil}, nil}, &TreeNode{3, nil, nil}}, &TreeNode{-3, &TreeNode{-2, nil, nil}, nil}}, 3})
	cases = append(cases, test_data{&TreeNode{1, &TreeNode{2, nil, nil}, nil}, 3})
	cases = append(cases, test_data{&TreeNode{2, &TreeNode{-1, nil, nil}, nil}, 2})
	cases = append(cases, test_data{&TreeNode{-2, &TreeNode{-1, nil, nil}, nil}, -1})
	cases = append(cases, test_data{&TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}, 6})
	cases = append(cases, test_data{&TreeNode{-10, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}, 42})

	for _, _case := range cases {
		assert.Equal(t, _case.output, maxPathSum(_case.input))
	}

	/*
		btree := &TreeNode{1, &TreeNode{2, nil, nil}, nil}
		fmt.Println(maxPathSum(btree))
		btree = &TreeNode{2, &TreeNode{-1, nil, nil}, nil}
		fmt.Println(maxPathSum(btree))
		btree = &TreeNode{-2, &TreeNode{-1, nil, nil}, nil}
		fmt.Println(maxPathSum(btree))
		btree = &TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}
		fmt.Println(maxPathSum(btree))
		btree = &TreeNode{-10, &TreeNode{9, nil, nil}, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{7, nil, nil}}}
		fmt.Println(maxPathSum(btree))
	*/
}
