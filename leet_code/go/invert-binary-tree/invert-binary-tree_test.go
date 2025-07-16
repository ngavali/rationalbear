package main

import (
	"fmt"
	"testing"
)

func TestInvertTree(t *testing.T) {

    inputs := []*TreeNode{
        {4, &TreeNode{2, &TreeNode{1,nil,nil}, &TreeNode{3,nil,nil}}, &TreeNode{7,&TreeNode{8,nil,nil},&TreeNode{9,nil,nil}}},
        {2,&TreeNode{1,nil,nil}, &TreeNode{3,nil,nil}},
    }

    for i, input := range inputs {
        fmt.Printf("%d->%+v\n", i, input)
        got := invertTree(input)
        fmt.Printf("%d->%+v\n", i, got )
    }
}
