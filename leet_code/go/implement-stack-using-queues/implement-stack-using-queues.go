package main

type ListNode struct {
	Val  int
	Next *ListNode
}

type MyStack struct {
	head *ListNode
}

func Constructor() MyStack {
	return MyStack{nil}
}

func (this *MyStack) Push(x int) {
	node := &ListNode{x, this.head}
    this.head = node
}

func (this *MyStack) Pop() int {
	popped := this.head.Val
	this.head = this.head.Next
	return popped
}

func (this *MyStack) Top() int {
	return this.head.Val
}

func (this *MyStack) Empty() bool {
	return this.head == nil
}

/**
 * Your MyStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * param_2 := obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.Empty();
 */
