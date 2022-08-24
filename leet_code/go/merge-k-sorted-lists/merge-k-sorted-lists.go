package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeKLists(lists []*ListNode) *ListNode {

	if len(lists) == 0 {
		return nil
	}

	listIndex := 0

	var sortedListHead, sortedList *ListNode
	is_lists_empty := false

	for !is_lists_empty {

		lowest := 10000
		for i := 0; i < len(lists); i++ {
			tmp := lists[i]

			if tmp != nil {
				if tmp.Val < lowest {
					listIndex = i
					lowest = tmp.Val
				}
			}
		}

		if lowest != 10000 {
			if sortedList == nil {
				sortedList = lists[listIndex]
				sortedListHead = sortedList
			} else {
				sortedList.Next = lists[listIndex]
				sortedList = sortedList.Next
			}

			lists[listIndex] = lists[listIndex].Next
		}

		is_lists_empty = true
		for i := range lists {
			if lists[i] != nil {
				is_lists_empty = false
				break
			}
		}
	}

	return sortedListHead

}

func main() {

	//L1 := &ListNode{3, &ListNode{4, &ListNode{5, nil}}}
	//L2 := &ListNode{2, &ListNode{3, &ListNode{4, nil}}}
	//L3 := &ListNode{1, &ListNode{6, nil}}
	//fmt.Println(mergeKLists([]*ListNode{L1, L2, L3}))
	fmt.Println(mergeKLists([]*ListNode{{}, {}, {}}))
}
