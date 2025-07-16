package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {

	var mid, end, prev *ListNode = head, head, nil

	for end != nil && end.Next != nil {
		end = end.Next.Next
		mid = mid.Next
	}

	prev = nil

	for mid != nil {
		next := mid.Next
		mid.Next = prev
		prev = mid
		mid = next
	}

	mid = prev

	for head != nil && mid != nil {
		if head.Val != mid.Val {
			return false
		}
		head = head.Next
		mid = mid.Next
	}

	return true

}
