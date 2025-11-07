package main
/**
 * Definition for a Node.
 */
 type Node struct {
     Val int
     Prev *Node
     Next *Node
     Child *Node
 }


func dfs(root *Node) {
    if root!= nil {
        //fmt.Printf("%v ", root.Val)
        if root.Child!= nil {
            dfs(root.Child)
        }
        dfs(root.Next)
        if root.Child != nil {
            save_next := root.Next
            lr := root.Child
            for lr.Next != nil {
                lr = lr.Next
            }
            lr.Next = save_next
            if save_next != nil {
                save_next.Prev = lr
            }
            root.Next = root.Child
            root.Child.Prev = root
            root.Child = nil
        }
    }
}

func flatten(root *Node) *Node {
    dfs(root)
    ll := root
    for ll != nil {
        ll = ll.Next
    }
    return root
}

