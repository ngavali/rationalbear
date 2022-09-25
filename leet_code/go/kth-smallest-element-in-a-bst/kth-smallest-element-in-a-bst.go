package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var ans, position int
var m bool

func inorder(root *TreeNode, kth int) {
    if root != nil && !m {
        inorder(root.Left, kth)
        position++
        if position == kth {
            ans = root.Val
            m = true
            return
        }
        inorder(root.Right, kth)
    }
}

func kthSmallest(root *TreeNode, k int) int {
    ans, position, m = 0, 0, false
    inorder(root, k)
        return ans
}
