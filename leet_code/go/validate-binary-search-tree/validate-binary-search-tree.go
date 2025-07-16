package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func validateBSTWithPointer(root, l, r *TreeNode) bool {

	if root == nil {
		return true
	}
	if l != nil && root.Val <= l.Val {
		return false
	}
	if r != nil && root.Val >= r.Val {
		return false
	}

	return validateBSTWithPointer(root.Left, l, root) && validateBSTWithPointer(root.Right, root, r)

}

func validateBSTWithDFSInOrder(root *TreeNode, prev *TreeNode) bool {

	if root != nil {

		if !validateBSTWithDFSInOrder(root.Left, prev) {
			return false
		}
		if prev != nil && root.Val <= prev.Val {
			return false
		}
		prev = root
		return validateBSTWithDFSInOrder(root.Right, prev)

	}
	return true
}

func validateBST(root *TreeNode, min, max int) bool {

	if root == nil {
		return true
	}
	if root.Val <= min || root.Val >= max {
		return false
	}
	return validateBST(root.Left, min, root.Val) && validateBST(root.Right, root.Val, max)

}

func isValidBST(root *TreeNode) bool {
	return validateBST(root, math.MinInt, math.MaxInt)
	//return validateBSTWithPointer(root, nil, nil)
	//return validateBSTWithDFSInOrder(root, nil)
    /*
    prev = nil
    return isValidBSTUtil(root)
    */
}

var prev *TreeNode = nil

func isValidBSTUtil(root *TreeNode) bool {

	if root != nil {

		if !isValidBSTUtil(root.Left) {
			return false
		}
		if prev != nil && root.Val <= prev.Val {
			return false
		}
		prev = root
		return isValidBSTUtil(root.Right)

	}

	return true

}

func main() {
	fmt.Println(isValidBST(&TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}))
}
