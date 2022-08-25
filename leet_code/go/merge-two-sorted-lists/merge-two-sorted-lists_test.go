package main

import (
	"testing"
)

func verifyLL( ll1, ll2 *ListNode) bool {

    for ll1!= nil || ll2 != nil {
        if ll2 == nil || ll1 == nil || ll1.Val != ll2.Val {
            return false
        }
        ll1 = ll1.Next
        ll2 = ll2.Next
    }
    return true
}

func TestMergeTwoLists(t *testing.T) {

    inputs := [][]*ListNode{ 
        {
            {1, &ListNode{2, &ListNode{4, nil}}},
            {1, &ListNode{3, &ListNode{4, nil}}},
        },
        {
            {1,&ListNode{10,&ListNode{20, nil}}},
            {2,&ListNode{5,&ListNode{25, nil}}},
        },
    }

    outputs := []*ListNode{
        {1, &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{4, nil}}}}}},
        {1, &ListNode{2, &ListNode{5, &ListNode{10, &ListNode{20, &ListNode{25, nil}}}}}},
    }

    for i, input := range inputs {
        if !verifyLL(outputs[i], mergeTwoLists(input[0], input[1])) {
            t.Errorf("Test for input %d failed", i)
        }
    }


}
