package main

import "fmt"

type TreeNode struct {
    Val   int
    Left  *TreeNode
    Right *TreeNode
}

func levelOrder(root *TreeNode) (result [][]int) {

    if root == nil {
        return
    }

    queue := make([]*TreeNode, 0, 2048)
    queue = append(queue, root)

    for len(queue) > 0 {

        N := len(queue)
        level := make([]int, 0 ,N*2)
        for i:=0; i<N; i++ {
            node :=  queue[i]
            level = append(level, node.Val)
            if node.Left != nil {
                queue = append(queue, node.Left)
            }
            if node.Right != nil {
                queue = append(queue, node.Right)
            }
        }

        queue = queue[N:]

        result = append(result, level)

    }

    return
}

func XlevelOrder(root *TreeNode) (result [][]int) {

    if root == nil {
        return nil
    }

    queue := []*TreeNode{root}

    for len(queue) > 0 {

        levelQueue := []*TreeNode{}
        levelNodes := []int{}
        for _, node := range queue {
            levelNodes = append(levelNodes, node.Val)
            if node.Left != nil {
                levelQueue = append(levelQueue, node.Left)
            }
            if node.Right != nil {
                levelQueue = append(levelQueue, node.Right)
            }
        }

        result = append(result, levelNodes)

        queue = levelQueue

    }

    return
}

func main() {
    input := &TreeNode{3,
    &TreeNode{9, nil, nil},
    &TreeNode{20, 
    &TreeNode{15, nil, nil}, 
    &TreeNode{7, nil, nil},
},
    }
    fmt.Println(levelOrder(input))

}
