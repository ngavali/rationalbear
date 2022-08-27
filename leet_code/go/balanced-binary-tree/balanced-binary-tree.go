package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxOf(x, y int) int {
	if x > y {
		return x
	}
	return y
}

var isTreeBalanced = true

func NewFindBalanced(root *TreeNode) bool {

	var stack []*TreeNode
	for root != nil || len(stack) > 0 {
		if root != nil {
			stack = append(stack, root)
			root = root.Left
		} else {
			if len(stack) > 0 {
				root = stack[len(stack)-1]
				stack = stack[:len(stack)-1]
                fmt.Println(root.Val)
			}
			root = root.Right
		}
	}
	return isTreeBalanced
}

func getTreeHeight(root *TreeNode) int {

	maxHeight := 0
	if root != nil && isTreeBalanced {

		leftHeight := getTreeHeight(root.Left)
		rightHeight := getTreeHeight(root.Right)

		if leftHeight-rightHeight > 1 || rightHeight-leftHeight > 1 {
			isTreeBalanced = false
		}
		maxHeight = maxOf(leftHeight, rightHeight)

	}

	return maxHeight + 1

}

func isBalanced(root *TreeNode) bool {

	isTreeBalanced = true

	if root != nil {

		getTreeHeight(root)

	}

	return isTreeBalanced

}
