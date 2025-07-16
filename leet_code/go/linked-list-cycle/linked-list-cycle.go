package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {

	fast := head

	for fast != nil && fast.Next != nil {
		head = head.Next
		fast = fast.Next.Next

		if head.Val == fast.Val {
			return true
		}
	}

	return false
}

func main() {
	lnode := &ListNode{2,nil}
    lnode.Next = &ListNode{0, &ListNode{4, lnode}}
	ll := &ListNode{3, lnode}

	fmt.Println(
		hasCycle(ll),
	)
}
