package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) (rightSideViewArray []int) {

	if root != nil {

		q := []*TreeNode{}
		q = append(q, root)

		for len(q) > 0 {
			rightSideViewArray = append(rightSideViewArray, q[len(q)-1].Val)

			N := len(q)
			for i := 0; i < N; i++ {

				n := q[0]

				if n.Left != nil {
					q = append(q, n.Left)
				}
				if n.Right != nil {
					q = append(q, n.Right)
				}

				q = q[1:]

			}
		}
	}
	return

}
