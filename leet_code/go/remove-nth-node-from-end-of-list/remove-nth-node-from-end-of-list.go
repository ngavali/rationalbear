package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {

	position := 0
	follower := head

	link := head
	for link != nil {
		fmt.Println(link.Val)
		link = link.Next
		if position > n {
			follower = follower.Next
		}
		position++
	}

	if position == 0 {
		return nil
	}
	follower.Next = follower.Next.Next
	return head
}

func main() {
	head := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, nil}}}}}
	fmt.Println(removeNthFromEnd(head, 2))
	//head := &ListNode{1, &ListNode{2, nil}}
	//fmt.Println(removeNthFromEnd(head, 1))
	fmt.Println("After removal")
	for head != nil {
		fmt.Println(head.Val)
		head = head.Next
	}
}
