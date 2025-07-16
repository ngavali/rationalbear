package main

import "fmt"

type ListNode struct {
    Val  int
    Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {

    tmp := &ListNode{0, nil}
    head := tmp

    for {

        if list1 == nil {
            tmp.Next = list2
            break
        }

        if list2 == nil {
            tmp.Next = list1
            break
        }

        if list1.Val < list2.Val {
            tmp.Next = list1
            list1 = list1.Next
        } else {
            tmp.Next = list2
            list2 = list2.Next
        }

        tmp = tmp.Next

    }

    return head.Next

}

func main() {

    ll := mergeTwoLists(
        &ListNode{1, &ListNode{2, &ListNode{4, nil}}},
        &ListNode{1, &ListNode{3, &ListNode{4, nil}}},
    )
    /*
    ll := mergeTwoLists(
        &ListNode{1,&ListNode{10,&ListNode{20, nil}}},
        &ListNode{2,&ListNode{5,&ListNode{25, nil}}},
    )
    */

    for ll != nil {
        fmt.Println(ll.Val)
        ll = ll.Next
    }

}
