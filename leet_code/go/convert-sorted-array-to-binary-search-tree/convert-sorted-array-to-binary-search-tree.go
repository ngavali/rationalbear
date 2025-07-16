package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func fillBST(lo, hi int, nums []int) *TreeNode {

	if lo < hi {
		mid := lo + (hi-lo)/2

		root := &TreeNode{Val: nums[mid]}
		root.Left = fillBST(lo, mid, nums)
		root.Right = fillBST(mid+1, hi, nums)

		return root
	}

	return nil
}

func sortedArrayToBST(nums []int) *TreeNode {

	if len(nums) != 0 {
		mid := len(nums) / 2

		root := &TreeNode{Val: nums[mid]}
		root.Left = sortedArrayToBST(nums[:mid])
		root.Right = sortedArrayToBST(nums[mid+1:])

		return root
	}
	return nil
}

func _sortedArrayToBST(nums []int) *TreeNode {
	return fillBST(0, len(nums), nums)
}

func DFS_inorder(root *TreeNode, result *[]int) {
	if root != nil {
		DFS_inorder(root.Left, result)
		*result = append(*result, root.Val)
		DFS_inorder(root.Right, result)
	}
}

/*
func DFS_inorder(root *TreeNode, out io.Writer) {
	if root != nil {
		DFS_inorder(root.Left, out)
		fmt.Fprintln(out, root.Val)
		DFS_inorder(root.Right, out)
	}
}*/
