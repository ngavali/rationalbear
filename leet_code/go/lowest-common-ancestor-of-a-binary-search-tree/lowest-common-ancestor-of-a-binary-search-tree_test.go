package main

import "testing"

func TestLowestCommonAncestor(t *testing.T) {
	input := &TreeNode{6,
		&TreeNode{2, &TreeNode{0, nil, nil}, &TreeNode{4, &TreeNode{3, nil, nil}, &TreeNode{5, nil, nil}}},
		&TreeNode{8, &TreeNode{7, nil, nil}, &TreeNode{9, nil, nil}}}
	want := &TreeNode{2, nil, nil}
	got := lowestCommonAncestor(input, &TreeNode{2, nil, nil}, &TreeNode{4, nil, nil})
	if got.Val != want.Val {
		t.Errorf("want %+v but got %v\n", want, got)
	}
}
