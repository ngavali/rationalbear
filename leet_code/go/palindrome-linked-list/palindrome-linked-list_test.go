package main

import (
	"testing"
)

func TestIsPalindrome(t *testing.T) {

	inputs := []*ListNode{
		{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, &ListNode{6, nil}}}}}},
		{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{3, &ListNode{2, &ListNode{1, nil}}}}}}},
		{10, nil},
		{1, &ListNode{2, nil}},
		{1, &ListNode{1, nil}},
	}
	outputs := []bool{
		false,
		true,
		true,
		false,
		true,
	}
	for i, input := range inputs {
		want := outputs[i]
		got := isPalindrome(input)
		if got != want {
			t.Errorf("For input[%d] want=%t but got=%t", i, want, got)
		}
	}
}
