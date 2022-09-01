package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var MaxDiameter = 0

func getMaxLength(root *TreeNode) int {

	if root == nil {
		return -1
	}

	leftHeight := getMaxLength(root.Left)+1
	rightHeight := getMaxLength(root.Right)+1

	if MaxDiameter < leftHeight+rightHeight {
		MaxDiameter = leftHeight + rightHeight
	}

	if leftHeight > rightHeight {
		return leftHeight
	}

	return rightHeight

}

func diameterOfBinaryTree(root *TreeNode) int {
    MaxDiameter = 0

    getMaxLength(root)

    return MaxDiameter
}
