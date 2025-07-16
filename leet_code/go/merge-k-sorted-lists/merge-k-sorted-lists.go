package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	sortedList := &ListNode{}
	sortedListHead := sortedList

	for {

		if list1 == nil {
			sortedList.Next = list2
			return sortedListHead.Next
		}

		if list2 == nil {
			sortedList.Next = list1
			return sortedListHead.Next
		}

		if list1.Val < list2.Val {
			sortedList.Next = list1
			list1 = list1.Next
		} else {
			sortedList.Next = list2
			list2 = list2.Next
		}
		sortedList = sortedList.Next

	}

}

func mergeKLists(lists []*ListNode) *ListNode {

	if len(lists) == 0 {
		return nil
	}

	sortedList := lists[0]
	for i := 1; i < len(lists); i++ {
		sortedList = mergeTwoLists(sortedList, lists[i])
	}

	return sortedList

}

func XmergeKLists(lists []*ListNode) *ListNode {

	if len(lists) == 0 {
		return nil
	}

	sortedList := &ListNode{0, nil}
	sortedListHead := sortedList

	for {
		listIndex := 0
		lowest := 10001
		updated := false

		for i, l := range lists {
			if l != nil && l.Val < lowest {
				listIndex = i
				lowest = l.Val
				updated = true
			}
		}

		if !updated {
			break
		}

		sortedList.Next = lists[listIndex]
		sortedList = sortedList.Next
		lists[listIndex] = lists[listIndex].Next
	}

	return sortedListHead.Next

}

func main() {

	L1 := &ListNode{3, &ListNode{4, &ListNode{5, nil}}}
	L2 := &ListNode{2, &ListNode{3, &ListNode{4, nil}}}
	L3 := &ListNode{1, &ListNode{6, nil}}
	fmt.Println(mergeKLists([]*ListNode{L1, L2, L3}))
	fmt.Println(mergeKLists([]*ListNode{nil, nil, nil}))

}
