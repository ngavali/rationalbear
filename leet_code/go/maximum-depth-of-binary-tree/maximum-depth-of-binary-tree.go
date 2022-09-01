package main

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

func maxDepth(root *TreeNode) int {

    if root!= nil {

        return maxOf(maxDepth(root.Left),maxDepth(root.Right))+1

    }

    return 0

}
