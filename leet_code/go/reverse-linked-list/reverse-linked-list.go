package main

import "fmt"

type ListNode struct {
    Val int
    Next *ListNode
}

func reverseList(head *ListNode) *ListNode {

    var prev *ListNode

    for head!= nil {
        next := head.Next
        head.Next = prev
        prev = head
        head = next
    }
    return prev
}

func XreverseList(head *ListNode) *ListNode {
    res := head

	for head != nil && head.Next != nil {
		temp := head.Next
		head.Next = head.Next.Next
		temp.Next = res
		res = temp
	}

	return res
}

func main() {

    head := &ListNode{1, &ListNode{2, &ListNode{3, nil}}}
    head = reverseList(head)
    for head!= nil {
        fmt.Println(head)
        head = head.Next
    }
}
