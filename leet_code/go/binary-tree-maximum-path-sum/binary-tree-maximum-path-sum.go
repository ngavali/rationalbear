package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func MaxOf(x, y int) int {
	if x > y {
		return x
	}
	return y
}

var MaxValue int

func MaxValFinder(root *TreeNode) int {

	if root != nil {
		left := MaxOf(MaxValFinder(root.Left), 0)
		right := MaxOf(MaxValFinder(root.Right), 0)
		sum := left + right + root.Val
		MaxValue = MaxOf(MaxValue, sum)
		return MaxOf(left, right) + root.Val
	}
	return 0

}

func maxPathSum(root *TreeNode) int {
	MaxValue = root.Val
	MaxValFinder(root)
	return MaxValue
}
