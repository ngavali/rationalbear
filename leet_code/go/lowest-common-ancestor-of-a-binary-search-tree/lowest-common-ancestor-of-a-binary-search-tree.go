package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func _lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {

	fmt.Printf("@%+v\n", root)
	if root == nil {
		return nil
	}

	if root.Val == p.Val || root.Val == q.Val {
		return root
	}

	left := lowestCommonAncestor(root.Left, p, q)
	right := lowestCommonAncestor(root.Right, p, q)

	if left != nil && right != nil {
		return root
	}

	if left != nil {
		return left
	} else {
		return right
	}

}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {

	if p.Val < root.Val && q.Val < root.Val {
		return lowestCommonAncestor(root.Left, p, q)
	}
	if p.Val > root.Val && q.Val > root.Val {
		return lowestCommonAncestor(root.Right, p, q)
	}

	return root

}

func main() {

	input := &TreeNode{6,
		&TreeNode{2, &TreeNode{0, nil, nil}, &TreeNode{4, &TreeNode{3, nil, nil}, &TreeNode{5, nil, nil}}},
		&TreeNode{8, &TreeNode{7, nil, nil}, &TreeNode{9, nil, nil}}}
	want := &TreeNode{2, nil, nil}
	got := lowestCommonAncestor(input, &TreeNode{2, nil, nil}, &TreeNode{4, nil, nil})
	fmt.Println(want, got)

}
